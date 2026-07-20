use margaret_attributes::attribute_index::AttributeIndex;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::model_codegen_error::ModelCodegenError;
use crate::models::models;
use crate::render::render;

pub fn render_models(index: &AttributeIndex) -> Result<GeneratedModuleTokens, ModelCodegenError> {
    let models = models(index)?;

    Ok(GeneratedModuleTokens::new("schema", render(&models)))
}
