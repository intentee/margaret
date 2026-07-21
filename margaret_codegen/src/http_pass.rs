use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn http_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    let capabilities = context.capabilities();

    if !capabilities.has_http && !capabilities.has_websockets {
        return Ok(());
    }

    let artifacts = margaret_http_codegen::render_http::render_http(
        context.index(),
        capabilities.has_views,
        context.websocket_servers(),
        context.middleware_plans(),
    )?;

    context.set_servers(artifacts.servers().to_vec());
    context.extend_modules(artifacts.into_modules());

    Ok(())
}
