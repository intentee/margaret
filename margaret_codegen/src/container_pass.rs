use margaret_container::container_bindings::ContainerBindings;
use margaret_container::rendered_container::RenderedContainer;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn container_pass(
    context: &mut BuildContext,
) -> Result<ContainerBindings, CodegenError> {
    let RenderedContainer { bindings, modules } =
        margaret_container::render_container::render_container(
            context.index(),
            context.provided_singletons(),
        )?;

    context.extend_modules(modules);

    Ok(bindings)
}
