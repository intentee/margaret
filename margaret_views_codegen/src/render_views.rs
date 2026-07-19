use margaret_attributes::attribute_index::AttributeIndex;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::render::render;
use crate::render_build::render_build;
use crate::views::views;
use crate::views_codegen_error::ViewsCodegenError;

pub fn render_views(
    index: &AttributeIndex,
) -> Result<Vec<GeneratedModuleTokens>, ViewsCodegenError> {
    let views = views(index)?;

    Ok(vec![
        GeneratedModuleTokens::new("views", render(&views)),
        GeneratedModuleTokens::new("views/build", render_build(&views)),
    ])
}
