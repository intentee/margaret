use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn websocket_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    registries: &BindingRegistries,
) -> Result<(), CodegenError> {
    if !context.capabilities().has_websockets {
        return Ok(());
    }

    let artifacts = margaret_websocket_codegen::render_websocket::render_websocket(
        context.index(),
        bindings,
        context.capabilities().has_views,
        context.middleware_plans(),
        registries,
    )?;

    context.set_websocket_servers(artifacts.servers);
    context.set_websocket_server_arguments(artifacts.server_console_arguments);
    context.extend_modules(artifacts.modules);

    Ok(())
}
