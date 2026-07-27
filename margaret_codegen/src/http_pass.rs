use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

use crate::build_context::BuildContext;
use crate::codegen_error::CodegenError;

pub(crate) fn http_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    registries: &BindingRegistries,
) -> Result<Vec<CanonicalPath>, CodegenError> {
    let capabilities = context.capabilities();

    if !capabilities.has_http && !capabilities.has_websockets {
        return Ok(Vec::new());
    }

    let artifacts = margaret_http_codegen::render_http::render_http(
        context.index(),
        capabilities.has_views,
        context.websocket_servers(),
        context.middleware_plans(),
        bindings,
        context.websocket_server_arguments(),
        registries,
    )?;

    let construction_roots = artifacts.construction_roots().to_vec();
    context.set_servers(artifacts.servers().to_vec());
    context.set_server_console_arguments(artifacts.server_console_arguments().clone());
    context.extend_modules(artifacts.into_modules());

    Ok(construction_roots)
}
