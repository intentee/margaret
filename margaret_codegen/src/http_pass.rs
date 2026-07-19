use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn http_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().has_http {
        return Ok(());
    }

    let artifacts = margaret_http_codegen::render_http::render_http(
        context.index(),
        context.capabilities().has_views,
    )?;

    context.set_servers(artifacts.servers().to_vec());
    context.extend_modules(artifacts.into_modules());

    Ok(())
}
