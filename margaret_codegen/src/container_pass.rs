use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn container_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    let modules = margaret_container::render_container::render_container(context.index())?;

    context.extend_modules(modules);

    Ok(())
}
