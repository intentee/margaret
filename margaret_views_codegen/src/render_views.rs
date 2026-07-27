use margaret_attributes::attribute_index::AttributeIndex;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::render::render;
use crate::render_build::render_build;
use crate::view::View;
use crate::views::views;
use crate::views_artifacts::ViewsArtifacts;
use crate::views_codegen_error::ViewsCodegenError;

fn views_console_arguments(
    views: &[View],
    bindings: &ContainerBindings,
) -> Result<Vec<ConsoleArgument>, ViewsCodegenError> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for view in views {
        collected.extend_from_slice(bindings.console_arguments(&view.concrete_path)?);
    }

    bindings.console_union(&collected).map_err(Into::into)
}

pub fn render_views(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<ViewsArtifacts, ViewsCodegenError> {
    let views = views(index)?;
    let console_arguments = views_console_arguments(&views, bindings)?;
    let retained_roots = views
        .iter()
        .map(|view| view.concrete_path.clone())
        .collect();
    Ok(ViewsArtifacts {
        modules: vec![
            GeneratedModuleTokens::new("views", render(&views)),
            GeneratedModuleTokens::new("views/build", render_build(&views, bindings)),
        ],
        console_arguments,
        retained_roots,
    })
}
