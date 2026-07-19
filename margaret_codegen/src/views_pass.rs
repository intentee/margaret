use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn views_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    let capabilities = context.capabilities();

    if !(capabilities.has_views && capabilities.has_http) {
        return Ok(());
    }

    let modules = margaret_views_codegen::render_views::render_views(context.index())?;

    context.extend_modules(modules);

    Ok(())
}
