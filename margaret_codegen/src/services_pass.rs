use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn services_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().serves {
        return Ok(());
    }

    let module = margaret_service_codegen::render_services::render_services(
        context.index(),
        context.servers(),
    )?;

    context.extend_modules(vec![module]);

    Ok(())
}
