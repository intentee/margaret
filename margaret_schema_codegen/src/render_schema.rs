use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::framework_tables::FrameworkTables;
use crate::render::render;

#[must_use]
pub fn render_schema(models: &[Model], framework_tables: FrameworkTables) -> GeneratedModuleTokens {
    GeneratedModuleTokens::new("schema", render(models, framework_tables))
}
