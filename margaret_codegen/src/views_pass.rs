use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn views_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
) -> Result<Vec<CanonicalPath>, CodegenError> {
    let capabilities = context.capabilities();

    if !capabilities.has_views {
        return Ok(Vec::new());
    }

    let artifacts = margaret_views_codegen::render_views::render_views(context.index(), bindings)?;

    let construction_roots = artifacts.construction_roots;
    context.set_views_console_arguments(artifacts.console_arguments);
    context.extend_modules(artifacts.modules);

    Ok(construction_roots)
}
