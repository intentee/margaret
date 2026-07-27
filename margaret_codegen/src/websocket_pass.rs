use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn websocket_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    registries: &BindingRegistries,
) -> Result<Vec<CanonicalPath>, CodegenError> {
    if !context.capabilities().has_websockets {
        return Ok(Vec::new());
    }

    let artifacts = margaret_websocket_codegen::render_websocket::render_websocket(
        context.index(),
        bindings,
        context.middleware_plans(),
        registries,
    )?;

    let construction_roots = artifacts.construction_roots;
    context.set_websocket_servers(artifacts.servers);
    context.set_websocket_server_arguments(artifacts.server_console_arguments);
    context.extend_modules(artifacts.modules);

    Ok(construction_roots)
}
