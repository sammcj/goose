use crate::recipe::read_recipe_file_content::read_parameter_file_content;
use crate::recipe::template_recipe::render_recipe_content_with_params;
use crate::recipe::validate_recipe::validate_recipe_template_from_content;
use crate::recipe::{
    Recipe, RecipeParameter, RecipeParameterInputType, RecipeParameterRequirement,
    BUILT_IN_RECIPE_DIR_PARAM,
};
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum RecipeError {
    #[error("Missing required parameters: {parameters:?}")]
    MissingParams { parameters: Vec<String> },
    #[error("Template rendering failed: {source}")]
    TemplateRendering { source: anyhow::Error },
    #[error("Recipe parsing failed: {source}")]
    RecipeParsing { source: anyhow::Error },
}

fn render_recipe_template<F>(
    recipe_content: String,
    recipe_dir: &Path,
    params: Vec<(String, String)>,
    user_prompt_fn: Option<F>,
) -> Result<(String, Vec<String>)>
where
    F: Fn(&str, &str) -> Result<String, anyhow::Error>,
{
    let recipe_dir_str = recipe_dir.display().to_string();

    let recipe_parameters =
        validate_recipe_template_from_content(&recipe_content, Some(recipe_dir_str.clone()))?
            .parameters;

    let (params_for_template, missing_params) =
        apply_values_to_parameters(&params, recipe_parameters, &recipe_dir_str, user_prompt_fn)?;

    let rendered_content = if missing_params.is_empty() {
        render_recipe_content_with_params(&recipe_content, &params_for_template)?
    } else {
        String::new()
    };

    Ok((rendered_content, missing_params))
}

pub fn build_recipe_from_template<F>(
    recipe_content: String,
    recipe_dir: &Path,
    params: Vec<(String, String)>,
    user_prompt_fn: Option<F>,
) -> Result<Recipe, RecipeError>
where
    F: Fn(&str, &str) -> Result<String, anyhow::Error>,
{
    let (rendered_content, missing_params) =
        render_recipe_template(recipe_content, recipe_dir, params.clone(), user_prompt_fn)
            .map_err(|source| RecipeError::TemplateRendering { source })?;

    if !missing_params.is_empty() {
        return Err(RecipeError::MissingParams {
            parameters: missing_params,
        });
    }

    let mut recipe = Recipe::from_content(&rendered_content)
        .map_err(|source| RecipeError::RecipeParsing { source })?;

    if let Some(ref mut sub_recipes) = recipe.sub_recipes {
        for sub_recipe in sub_recipes {
            sub_recipe.path = resolve_sub_recipe_path(&sub_recipe.path, recipe_dir)?;
        }
    }

    Ok(recipe)
}

pub fn apply_values_to_parameters<F>(
    user_params: &[(String, String)],
    recipe_parameters: Option<Vec<RecipeParameter>>,
    recipe_dir: &str,
    user_prompt_fn: Option<F>,
) -> Result<(HashMap<String, String>, Vec<String>)>
where
    F: Fn(&str, &str) -> Result<String, anyhow::Error>,
{
    let mut param_map: HashMap<String, String> = user_params.iter().cloned().collect();
    param_map.insert(
        BUILT_IN_RECIPE_DIR_PARAM.to_string(),
        recipe_dir.to_string(),
    );
    let mut missing_params: Vec<String> = Vec::new();
    for param in recipe_parameters.unwrap_or_default() {
        if !param_map.contains_key(&param.key) {
            match (&param.default, &param.requirement) {
                (Some(default), _) => param_map.insert(param.key.clone(), default.clone()),
                (None, RecipeParameterRequirement::UserPrompt) if user_prompt_fn.is_some() => {
                    let input_value =
                        user_prompt_fn.as_ref().unwrap()(&param.key, &param.description)?;
                    param_map.insert(param.key.clone(), input_value)
                }
                _ => {
                    missing_params.push(param.key.clone());
                    None
                }
            };
        } else if matches!(param.input_type, RecipeParameterInputType::File) {
            let file_path = param_map.get(&param.key).unwrap();
            let file_content = read_parameter_file_content(file_path)?;
            param_map.insert(param.key.clone(), file_content);
        }
    }
    Ok((param_map, missing_params))
}

fn resolve_sub_recipe_path(
    sub_recipe_path: &str,
    parent_recipe_dir: &Path,
) -> Result<String, RecipeError> {
    let path = if Path::new(sub_recipe_path).is_absolute() {
        Path::new(sub_recipe_path).to_path_buf()
    } else {
        parent_recipe_dir.join(sub_recipe_path)
    };
    if !path.exists() {
        return Err(RecipeError::RecipeParsing {
            source: anyhow::anyhow!("Sub-recipe file does not exist: {}", path.display()),
        });
    }

    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests;
