use anyhow::Result;
use async_stream::try_stream;
use async_trait::async_trait;
use futures::TryStreamExt;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::time::Duration;
use tokio::pin;
use tokio_util::io::StreamReader;

use super::base::{ConfigKey, MessageStream, Provider, ProviderMetadata, ProviderUsage, Usage};
use super::embedding::EmbeddingCapable;
use super::errors::ProviderError;
use super::formats::databricks::{create_request, response_to_message};
use super::oauth;
use super::utils::{get_model, ImageFormat};
use crate::config::ConfigError;
use crate::impl_provider_default;
use crate::message::Message;
use crate::model::ModelConfig;
use crate::providers::formats::openai::{get_usage, response_to_streaming_message};
use rmcp::model::Tool;
use serde_json::json;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};
use url::Url;

const DEFAULT_CLIENT_ID: &str = "databricks-cli";
const DEFAULT_REDIRECT_URL: &str = "http://localhost:8020";
// "offline_access" scope is used to request an OAuth 2.0 Refresh Token
// https://openid.net/specs/openid-connect-core-1_0.html#OfflineAccess
const DEFAULT_SCOPES: &[&str] = &["all-apis", "offline_access"];

/// Default timeout for API requests in seconds
const DEFAULT_TIMEOUT_SECS: u64 = 600;
/// Default initial interval for retry (in milliseconds)
const DEFAULT_INITIAL_RETRY_INTERVAL_MS: u64 = 5000;
/// Default maximum number of retries
const DEFAULT_MAX_RETRIES: usize = 6;
/// Default retry backoff multiplier
const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;
/// Default maximum interval for retry (in milliseconds)
const DEFAULT_MAX_RETRY_INTERVAL_MS: u64 = 320_000;

pub const DATABRICKS_DEFAULT_MODEL: &str = "databricks-claude-3-7-sonnet";
// Databricks can passthrough to a wide range of models, we only provide the default
pub const DATABRICKS_KNOWN_MODELS: &[&str] = &[
    "databricks-meta-llama-3-3-70b-instruct",
    "databricks-meta-llama-3-1-405b-instruct",
    "databricks-dbrx-instruct",
    "databricks-mixtral-8x7b-instruct",
];

pub const DATABRICKS_DOC_URL: &str =
    "https://docs.databricks.com/en/generative-ai/external-models/index.html";

/// Retry configuration for handling rate limit errors
#[derive(Debug, Clone)]
struct RetryConfig {
    /// Maximum number of retry attempts
    max_retries: usize,
    /// Initial interval between retries in milliseconds
    initial_interval_ms: u64,
    /// Multiplier for backoff (exponential)
    backoff_multiplier: f64,
    /// Maximum interval between retries in milliseconds
    max_interval_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            initial_interval_ms: DEFAULT_INITIAL_RETRY_INTERVAL_MS,
            backoff_multiplier: DEFAULT_BACKOFF_MULTIPLIER,
            max_interval_ms: DEFAULT_MAX_RETRY_INTERVAL_MS,
        }
    }
}

impl RetryConfig {
    /// Calculate the delay for a specific retry attempt (with jitter)
    fn delay_for_attempt(&self, attempt: usize) -> Duration {
        if attempt == 0 {
            return Duration::from_millis(0);
        }

        // Calculate exponential backoff
        let exponent = (attempt - 1) as u32;
        let base_delay_ms = (self.initial_interval_ms as f64
            * self.backoff_multiplier.powi(exponent as i32)) as u64;

        // Apply max limit
        let capped_delay_ms = std::cmp::min(base_delay_ms, self.max_interval_ms);

        // Add jitter (+/-20% randomness) to avoid thundering herd problem
        let jitter_factor = 0.8 + (rand::random::<f64>() * 0.4); // Between 0.8 and 1.2
        let jittered_delay_ms = (capped_delay_ms as f64 * jitter_factor) as u64;

        Duration::from_millis(jittered_delay_ms)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabricksAuth {
    Token(String),
    OAuth {
        host: String,
        client_id: String,
        redirect_url: String,
        scopes: Vec<String>,
    },
}

impl DatabricksAuth {
    /// Create a new OAuth configuration with default values
    pub fn oauth(host: String) -> Self {
        Self::OAuth {
            host,
            client_id: DEFAULT_CLIENT_ID.to_string(),
            redirect_url: DEFAULT_REDIRECT_URL.to_string(),
            scopes: DEFAULT_SCOPES.iter().map(|s| s.to_string()).collect(),
        }
    }
    pub fn token(token: String) -> Self {
        Self::Token(token)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DatabricksProvider {
    #[serde(skip)]
    client: Client,
    host: String,
    auth: DatabricksAuth,
    model: ModelConfig,
    image_format: ImageFormat,
    #[serde(skip)]
    retry_config: RetryConfig,
}

impl_provider_default!(DatabricksProvider);

impl DatabricksProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();

        // For compatibility for now we check both config and secret for databricks host
        // but it is not actually a secret value
        let mut host: Result<String, ConfigError> = config.get_param("DATABRICKS_HOST");
        if host.is_err() {
            host = config.get_secret("DATABRICKS_HOST")
        }

        if host.is_err() {
            return Err(ConfigError::NotFound(
                "Did not find DATABRICKS_HOST in either config file or keyring".to_string(),
            )
            .into());
        }

        let host = host?;

        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        // Load optional retry configuration from environment
        let retry_config = Self::load_retry_config(config);

        // If we find a databricks token we prefer that
        if let Ok(api_key) = config.get_secret("DATABRICKS_TOKEN") {
            return Ok(Self {
                client,
                host,
                auth: DatabricksAuth::token(api_key),
                model,
                image_format: ImageFormat::OpenAi,
                retry_config,
            });
        }

        // Otherwise use Oauth flow
        Ok(Self {
            client,
            auth: DatabricksAuth::oauth(host.clone()),
            host,
            model,
            image_format: ImageFormat::OpenAi,
            retry_config,
        })
    }

    /// Loads retry configuration from environment variables or uses defaults.
    fn load_retry_config(config: &crate::config::Config) -> RetryConfig {
        let max_retries = config
            .get_param("DATABRICKS_MAX_RETRIES")
            .ok()
            .and_then(|v: String| v.parse::<usize>().ok())
            .unwrap_or(DEFAULT_MAX_RETRIES);

        let initial_interval_ms = config
            .get_param("DATABRICKS_INITIAL_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_INITIAL_RETRY_INTERVAL_MS);

        let backoff_multiplier = config
            .get_param("DATABRICKS_BACKOFF_MULTIPLIER")
            .ok()
            .and_then(|v: String| v.parse::<f64>().ok())
            .unwrap_or(DEFAULT_BACKOFF_MULTIPLIER);

        let max_interval_ms = config
            .get_param("DATABRICKS_MAX_RETRY_INTERVAL_MS")
            .ok()
            .and_then(|v: String| v.parse::<u64>().ok())
            .unwrap_or(DEFAULT_MAX_RETRY_INTERVAL_MS);

        RetryConfig {
            max_retries,
            initial_interval_ms,
            backoff_multiplier,
            max_interval_ms,
        }
    }

    /// Create a new DatabricksProvider with the specified host and token
    ///
    /// # Arguments
    ///
    /// * `host` - The Databricks host URL
    /// * `token` - The Databricks API token
    ///
    /// # Returns
    ///
    /// Returns a Result containing the new DatabricksProvider instance
    pub fn from_params(host: String, api_key: String, model: ModelConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        Ok(Self {
            client,
            host,
            auth: DatabricksAuth::token(api_key),
            model,
            image_format: ImageFormat::OpenAi,
            retry_config: RetryConfig::default(),
        })
    }

    async fn ensure_auth_header(&self) -> Result<String> {
        match &self.auth {
            DatabricksAuth::Token(token) => Ok(format!("Bearer {}", token)),
            DatabricksAuth::OAuth {
                host,
                client_id,
                redirect_url,
                scopes,
            } => {
                let token =
                    oauth::get_oauth_token_async(host, client_id, redirect_url, scopes).await?;
                Ok(format!("Bearer {}", token))
            }
        }
    }

    async fn post(&self, payload: &Value) -> Result<Value, ProviderError> {
        // Check if this is an embedding request by looking at the payload structure
        let is_embedding = payload.get("input").is_some() && payload.get("messages").is_none();
        let path = if is_embedding {
            // For embeddings, use the embeddings endpoint
            format!("serving-endpoints/{}/invocations", "text-embedding-3-small")
        } else {
            // For chat completions, use the model name in the path
            format!("serving-endpoints/{}/invocations", self.model.model_name)
        };

        match self.post_with_retry(path.as_str(), payload).await {
            Ok(res) => res.json().await.map_err(|_| {
                ProviderError::RequestFailed("Response body is not valid JSON".to_string())
            }),
            Err(e) => Err(e),
        }
    }

    async fn post_with_retry(
        &self,
        path: &str,
        payload: &Value,
    ) -> Result<reqwest::Response, ProviderError> {
        let base_url = Url::parse(&self.host)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;
        let url = base_url.join(path).map_err(|e| {
            ProviderError::RequestFailed(format!("Failed to construct endpoint URL: {e}"))
        })?;

        let mut attempts = 0;
        loop {
            let auth_header = self.ensure_auth_header().await?;
            let response = self
                .client
                .post(url.clone())
                .header("Authorization", auth_header)
                .json(payload)
                .send()
                .await?;

            let status = response.status();

            break match status {
                StatusCode::OK => Ok(response),
                StatusCode::TOO_MANY_REQUESTS
                | StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::SERVICE_UNAVAILABLE => {
                    if attempts < self.retry_config.max_retries {
                        attempts += 1;
                        tracing::warn!(
                            "{}: retrying ({}/{})",
                            status,
                            attempts,
                            self.retry_config.max_retries
                        );

                        let delay = self.retry_config.delay_for_attempt(attempts);
                        tracing::info!("Backing off for {:?} before retry", delay);
                        sleep(delay).await;

                        continue;
                    }

                    Err(match status {
                        StatusCode::TOO_MANY_REQUESTS => {
                            ProviderError::RateLimitExceeded("Rate limit exceeded".to_string())
                        }
                        _ => ProviderError::ServerError("Server error".to_string()),
                    })
                }
                StatusCode::BAD_REQUEST => {
                    // Databricks provides a generic 'error' but also includes 'external_model_message' which is provider specific
                    // We try to extract the error message from the payload and check for phrases that indicate context length exceeded
                    let bytes = response.bytes().await?;
                    let payload_str = String::from_utf8_lossy(&bytes).to_lowercase();
                    let check_phrases = [
                        "too long",
                        "context length",
                        "context_length_exceeded",
                        "reduce the length",
                        "token count",
                        "exceeds",
                        "exceed context limit",
                        "input length",
                        "max_tokens",
                        "decrease input length",
                        "context limit",
                    ];
                    if check_phrases.iter().any(|c| payload_str.contains(c)) {
                        return Err(ProviderError::ContextLengthExceeded(payload_str));
                    }

                    let mut error_msg = "Unknown error".to_string();
                    if let Ok(response_json) = serde_json::from_slice::<Value>(&bytes) {
                        // try to convert message to string, if that fails use external_model_message
                        error_msg = response_json
                            .get("message")
                            .and_then(|m| m.as_str())
                            .or_else(|| {
                                response_json
                                    .get("external_model_message")
                                    .and_then(|ext| ext.get("message"))
                                    .and_then(|m| m.as_str())
                            })
                            .unwrap_or("Unknown error")
                            .to_string();
                    }

                    tracing::debug!(
                        "{}",
                        format!(
                            "Provider request failed with status: {}. Payload: {:?}",
                            status, payload_str
                        )
                    );
                    return Err(ProviderError::RequestFailed(format!(
                        "Request failed with status: {}. Message: {}",
                        status, error_msg
                    )));
                }
                _ => {
                    tracing::debug!(
                        "{}",
                        format!(
                            "Provider request failed with status: {}. Payload: {:?}",
                            status,
                            response.text().await.ok().unwrap_or_default()
                        )
                    );
                    return Err(ProviderError::RequestFailed(format!(
                        "Request failed with status: {}",
                        status
                    )));
                }
            };
        }
    }
}

#[async_trait]
impl Provider for DatabricksProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "databricks",
            "Databricks",
            "Models on Databricks AI Gateway",
            DATABRICKS_DEFAULT_MODEL,
            DATABRICKS_KNOWN_MODELS.to_vec(),
            DATABRICKS_DOC_URL,
            vec![
                ConfigKey::new("DATABRICKS_HOST", true, false, None),
                ConfigKey::new("DATABRICKS_TOKEN", false, true, None),
            ],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        self.model.clone()
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        let mut payload = create_request(&self.model, system, messages, tools, &self.image_format)?;
        // Remove the model key which is part of the url with databricks
        payload
            .as_object_mut()
            .expect("payload should have model key")
            .remove("model");

        let response = self.post(&payload).await?;

        // Parse response
        let message = response_to_message(&response)?;
        let usage = response.get("usage").map(get_usage).unwrap_or_else(|| {
            tracing::debug!("Failed to get usage data");
            Usage::default()
        });
        let model = get_model(&response);
        super::utils::emit_debug_trace(&self.model, &payload, &response, &usage);

        Ok((message, ProviderUsage::new(model, usage)))
    }

    async fn stream(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<MessageStream, ProviderError> {
        let mut payload = create_request(&self.model, system, messages, tools, &self.image_format)?;
        // Remove the model key which is part of the url with databricks
        payload
            .as_object_mut()
            .expect("payload should have model key")
            .remove("model");

        payload
            .as_object_mut()
            .unwrap()
            .insert("stream".to_string(), Value::Bool(true));

        let response = self
            .post_with_retry(
                format!("serving-endpoints/{}/invocations", self.model.model_name).as_str(),
                &payload,
            )
            .await?;

        // Map reqwest error to io::Error
        let stream = response.bytes_stream().map_err(io::Error::other);

        let model_config = self.model.clone();
        // Wrap in a line decoder and yield lines inside the stream
        Ok(Box::pin(try_stream! {
            let stream_reader = StreamReader::new(stream);
            let framed = FramedRead::new(stream_reader, LinesCodec::new()).map_err(anyhow::Error::from);

            let message_stream = response_to_streaming_message(framed);
            pin!(message_stream);
            while let Some(message) = message_stream.next().await {
                let (message, usage) = message.map_err(|e| ProviderError::RequestFailed(format!("Stream decode error: {}", e)))?;
                super::utils::emit_debug_trace(&model_config, &payload, &message, &usage.as_ref().map(|f| f.usage).unwrap_or_default());
                yield (message, usage);
            }
        }))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_embeddings(&self) -> bool {
        true
    }

    async fn create_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, ProviderError> {
        EmbeddingCapable::create_embeddings(self, texts)
            .await
            .map_err(|e| ProviderError::ExecutionError(e.to_string()))
    }

    async fn fetch_supported_models_async(&self) -> Result<Option<Vec<String>>, ProviderError> {
        let base_url = Url::parse(&self.host)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;
        let url = base_url.join("api/2.0/serving-endpoints").map_err(|e| {
            ProviderError::RequestFailed(format!("Failed to construct endpoint URL: {e}"))
        })?;

        let auth_header = match self.ensure_auth_header().await {
            Ok(header) => header,
            Err(e) => {
                tracing::warn!("Failed to authorize with Databricks: {}", e);
                return Ok(None); // Return None to fall back to manual input
            }
        };

        let response = match self
            .client
            .get(url)
            .header("Authorization", auth_header)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                tracing::warn!("Failed to fetch Databricks models: {}", e);
                return Ok(None); // Return None to fall back to manual input
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            if let Ok(error_text) = response.text().await {
                tracing::warn!(
                    "Failed to fetch Databricks models: {} - {}",
                    status,
                    error_text
                );
            } else {
                tracing::warn!("Failed to fetch Databricks models: {}", status);
            }
            return Ok(None); // Return None to fall back to manual input
        }

        let json: Value = match response.json().await {
            Ok(json) => json,
            Err(e) => {
                tracing::warn!("Failed to parse Databricks API response: {}", e);
                return Ok(None);
            }
        };

        let endpoints = match json.get("endpoints").and_then(|v| v.as_array()) {
            Some(endpoints) => endpoints,
            None => {
                tracing::warn!(
                    "Unexpected response format from Databricks API: missing 'endpoints' array"
                );
                return Ok(None);
            }
        };

        let models: Vec<String> = endpoints
            .iter()
            .filter_map(|endpoint| {
                endpoint
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|name| name.to_string())
            })
            .collect();

        if models.is_empty() {
            tracing::debug!("No serving endpoints found in Databricks workspace");
            Ok(None)
        } else {
            tracing::debug!(
                "Found {} serving endpoints in Databricks workspace",
                models.len()
            );
            Ok(Some(models))
        }
    }
}

#[async_trait]
impl EmbeddingCapable for DatabricksProvider {
    async fn create_embeddings(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        if texts.is_empty() {
            return Ok(vec![]);
        }

        // Create request in Databricks format for embeddings
        let request = json!({
            "input": texts,
        });

        let response = self.post(&request).await?;

        let embeddings = response["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format: missing data array"))?
            .iter()
            .map(|item| {
                item["embedding"]
                    .as_array()
                    .ok_or_else(|| anyhow::anyhow!("Invalid embedding format"))?
                    .iter()
                    .map(|v| v.as_f64().map(|f| f as f32))
                    .collect::<Option<Vec<f32>>>()
                    .ok_or_else(|| anyhow::anyhow!("Invalid embedding values"))
            })
            .collect::<Result<Vec<Vec<f32>>>>()?;

        Ok(embeddings)
    }
}
