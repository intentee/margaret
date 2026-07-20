use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn websocket_pass(context: &mut BuildContext) -> Result<(), CodegenError> {
    if !context.capabilities().has_websocket {
        return Ok(());
    }

    let artifacts =
        margaret_websocket_codegen::render_websocket::render_websocket(context.index())?;

    context.set_synthetic_routes(artifacts.synthetic_routes);
    context.extend_modules(artifacts.modules);

    Ok(())
}
