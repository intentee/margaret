use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn services_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    serve_arguments: &[ConsoleArgument],
) -> Result<(), CodegenError> {
    if !context.capabilities().serves {
        return Ok(());
    }

    let module = margaret_service_codegen::render_services::render_services(
        context.index(),
        context.servers(),
        context.capabilities().has_views,
        bindings,
        serve_arguments,
        context.server_console_arguments(),
        context.views_console_arguments(),
    )?;

    context.extend_modules(vec![module]);

    Ok(())
}
