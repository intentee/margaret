use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn console_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    serve_arguments: &[ConsoleArgument],
) -> Result<(), CodegenError> {
    if !context.capabilities().has_console {
        return Ok(());
    }

    let module = margaret_console_codegen::render_console::render_console(
        context.index(),
        context.capabilities().serves,
        context.capabilities().has_models,
        context.servers(),
        serve_arguments,
        bindings,
    )?;

    context.extend_modules(vec![module]);

    Ok(())
}
