use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn services_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().serves {
        return Ok(());
    }

    let rendered = margaret_service_codegen::render_services::render_services(
        context.index(),
        context.servers(),
    )?;

    context.set_serve_arguments(rendered.serve_arguments);
    context.extend_modules(vec![rendered.module]);

    Ok(())
}
