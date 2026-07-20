use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn console_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().has_console {
        return Ok(());
    }

    let module = margaret_console_codegen::render_console::render_console(
        context.index(),
        context.capabilities().serves,
        context.capabilities().has_models,
        context.servers(),
        context.serve_arguments(),
    )?;

    context.extend_modules(vec![module]);

    Ok(())
}
