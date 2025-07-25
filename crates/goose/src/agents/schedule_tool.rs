//! Schedule tool handlers for the Goose agent
//!
//! This module contains all the handlers for the schedule management platform tool,
//! including job creation, execution, monitoring, and session management.

use std::sync::Arc;

use chrono::Utc;
use mcp_core::{ToolError, ToolResult};
use rmcp::model::Content;

use crate::recipe::Recipe;
use crate::scheduler_trait::SchedulerTrait;

use super::Agent;

impl Agent {
    /// Handle schedule management tool calls
    pub async fn handle_schedule_management(
        &self,
        arguments: serde_json::Value,
        _request_id: String,
    ) -> ToolResult<Vec<Content>> {
        let scheduler = match self.scheduler_service.lock().await.as_ref() {
            Some(s) => s.clone(),
            None => {
                return Err(ToolError::ExecutionError(
                    "Scheduler not available. This tool only works in server mode.".to_string(),
                ))
            }
        };

        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'action' parameter".to_string()))?;

        match action {
            "list" => self.handle_list_jobs(scheduler).await,
            "create" => self.handle_create_job(scheduler, arguments).await,
            "run_now" => self.handle_run_now(scheduler, arguments).await,
            "pause" => self.handle_pause_job(scheduler, arguments).await,
            "unpause" => self.handle_unpause_job(scheduler, arguments).await,
            "delete" => self.handle_delete_job(scheduler, arguments).await,
            "kill" => self.handle_kill_job(scheduler, arguments).await,
            "inspect" => self.handle_inspect_job(scheduler, arguments).await,
            "sessions" => self.handle_list_sessions(scheduler, arguments).await,
            "session_content" => self.handle_session_content(arguments).await,
            _ => Err(ToolError::ExecutionError(format!(
                "Unknown action: {}",
                action
            ))),
        }
    }

    /// List all scheduled jobs
    async fn handle_list_jobs(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
    ) -> ToolResult<Vec<Content>> {
        match scheduler.list_scheduled_jobs().await {
            Ok(jobs) => {
                let jobs_json = serde_json::to_string_pretty(&jobs).map_err(|e| {
                    ToolError::ExecutionError(format!("Failed to serialize jobs: {}", e))
                })?;
                Ok(vec![Content::text(format!(
                    "Scheduled Jobs:\n{}",
                    jobs_json
                ))])
            }
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to list jobs: {}",
                e
            ))),
        }
    }

    /// Create a new scheduled job from a recipe file
    async fn handle_create_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let recipe_path = arguments
            .get("recipe_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ToolError::ExecutionError("Missing 'recipe_path' parameter".to_string())
            })?;

        let cron_expression = arguments
            .get("cron_expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ToolError::ExecutionError("Missing 'cron_expression' parameter".to_string())
            })?;

        // Get the execution_mode parameter, defaulting to "background" if not provided
        let execution_mode = arguments
            .get("execution_mode")
            .and_then(|v| v.as_str())
            .unwrap_or("background");

        // Validate execution_mode is either "foreground" or "background"
        if execution_mode != "foreground" && execution_mode != "background" {
            return Err(ToolError::ExecutionError(format!(
                "Invalid execution_mode: {}. Must be 'foreground' or 'background'",
                execution_mode
            )));
        }

        // Validate recipe file exists and is readable
        if !std::path::Path::new(recipe_path).exists() {
            return Err(ToolError::ExecutionError(format!(
                "Recipe file not found: {}",
                recipe_path
            )));
        }

        // Validate it's a valid recipe by trying to parse it
        match std::fs::read_to_string(recipe_path) {
            Ok(content) => {
                if recipe_path.ends_with(".json") {
                    serde_json::from_str::<Recipe>(&content).map_err(|e| {
                        ToolError::ExecutionError(format!("Invalid JSON recipe: {}", e))
                    })?;
                } else {
                    serde_yaml::from_str::<Recipe>(&content).map_err(|e| {
                        ToolError::ExecutionError(format!("Invalid YAML recipe: {}", e))
                    })?;
                }
            }
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Cannot read recipe file: {}",
                    e
                )))
            }
        }

        // Generate unique job ID
        let job_id = format!("agent_created_{}", Utc::now().timestamp());

        let job = crate::scheduler::ScheduledJob {
            id: job_id.clone(),
            source: recipe_path.to_string(),
            cron: cron_expression.to_string(),
            last_run: None,
            currently_running: false,
            paused: false,
            current_session_id: None,
            process_start_time: None,
            execution_mode: Some(execution_mode.to_string()),
        };

        match scheduler.add_scheduled_job(job).await {
            Ok(()) => Ok(vec![Content::text(format!(
                "Successfully created scheduled job '{}' for recipe '{}' with cron expression '{}' in {} mode",
                job_id, recipe_path, cron_expression, execution_mode
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to create job: {}",
                e
            ))),
        }
    }

    /// Run a scheduled job immediately
    async fn handle_run_now(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.run_now(job_id).await {
            Ok(session_id) => Ok(vec![Content::text(format!(
                "Successfully started job '{}'. Session ID: {}",
                job_id, session_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to run job: {}",
                e
            ))),
        }
    }

    /// Pause a scheduled job
    async fn handle_pause_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.pause_schedule(job_id).await {
            Ok(()) => Ok(vec![Content::text(format!(
                "Successfully paused job '{}'",
                job_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to pause job: {}",
                e
            ))),
        }
    }

    /// Resume a paused scheduled job
    async fn handle_unpause_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.unpause_schedule(job_id).await {
            Ok(()) => Ok(vec![Content::text(format!(
                "Successfully unpaused job '{}'",
                job_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to unpause job: {}",
                e
            ))),
        }
    }

    /// Delete a scheduled job
    async fn handle_delete_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.remove_scheduled_job(job_id).await {
            Ok(()) => Ok(vec![Content::text(format!(
                "Successfully deleted job '{}'",
                job_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to delete job: {}",
                e
            ))),
        }
    }

    /// Terminate a currently running job
    async fn handle_kill_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.kill_running_job(job_id).await {
            Ok(()) => Ok(vec![Content::text(format!(
                "Successfully killed running job '{}'",
                job_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to kill job: {}",
                e
            ))),
        }
    }

    /// Get information about a running job
    async fn handle_inspect_job(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        match scheduler.get_running_job_info(job_id).await {
            Ok(Some((session_id, start_time))) => {
                let duration = Utc::now().signed_duration_since(start_time);
                Ok(vec![Content::text(format!(
                    "Job '{}' is currently running:\n- Session ID: {}\n- Started: {}\n- Duration: {} seconds",
                    job_id, session_id, start_time.to_rfc3339(), duration.num_seconds()
                ))])
            }
            Ok(None) => Ok(vec![Content::text(format!(
                "Job '{}' is not currently running",
                job_id
            ))]),
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to inspect job: {}",
                e
            ))),
        }
    }

    /// List execution sessions for a job
    async fn handle_list_sessions(
        &self,
        scheduler: Arc<dyn SchedulerTrait>,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let job_id = arguments
            .get("job_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::ExecutionError("Missing 'job_id' parameter".to_string()))?;

        let limit = arguments
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;

        match scheduler.sessions(job_id, limit).await {
            Ok(sessions) => {
                if sessions.is_empty() {
                    Ok(vec![Content::text(format!(
                        "No sessions found for job '{}'",
                        job_id
                    ))])
                } else {
                    let sessions_info: Vec<String> = sessions
                        .into_iter()
                        .map(|(session_name, metadata)| {
                            format!(
                                "- Session: {} (Messages: {}, Working Dir: {})",
                                session_name,
                                metadata.message_count,
                                metadata.working_dir.display()
                            )
                        })
                        .collect();

                    Ok(vec![Content::text(format!(
                        "Sessions for job '{}':\n{}",
                        job_id,
                        sessions_info.join("\n")
                    ))])
                }
            }
            Err(e) => Err(ToolError::ExecutionError(format!(
                "Failed to list sessions: {}",
                e
            ))),
        }
    }

    /// Get the full content (metadata and messages) of a specific session
    async fn handle_session_content(
        &self,
        arguments: serde_json::Value,
    ) -> ToolResult<Vec<Content>> {
        let session_id = arguments
            .get("session_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                ToolError::ExecutionError("Missing 'session_id' parameter".to_string())
            })?;

        // Get the session file path
        let session_path = match crate::session::storage::get_path(
            crate::session::storage::Identifier::Name(session_id.to_string()),
        ) {
            Ok(path) => path,
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Invalid session ID '{}': {}",
                    session_id, e
                )));
            }
        };

        // Check if session file exists
        if !session_path.exists() {
            return Err(ToolError::ExecutionError(format!(
                "Session '{}' not found",
                session_id
            )));
        }

        // Read session metadata
        let metadata = match crate::session::storage::read_metadata(&session_path) {
            Ok(metadata) => metadata,
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Failed to read session metadata: {}",
                    e
                )));
            }
        };

        // Read session messages
        let messages = match crate::session::storage::read_messages(&session_path) {
            Ok(messages) => messages,
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Failed to read session messages: {}",
                    e
                )));
            }
        };

        // Format the response with metadata and messages
        let metadata_json = match serde_json::to_string_pretty(&metadata) {
            Ok(json) => json,
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Failed to serialize metadata: {}",
                    e
                )));
            }
        };

        let messages_json = match serde_json::to_string_pretty(&messages) {
            Ok(json) => json,
            Err(e) => {
                return Err(ToolError::ExecutionError(format!(
                    "Failed to serialize messages: {}",
                    e
                )));
            }
        };

        Ok(vec![Content::text(format!(
            "Session '{}' Content:\n\nMetadata:\n{}\n\nMessages:\n{}",
            session_id, metadata_json, messages_json
        ))])
    }
}
