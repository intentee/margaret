use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_client_module::JwksClientModule;
use margaret_jwks_codegen::jwks_server_module::JwksServerModule;
use margaret_jwks_codegen::render_jwks::render_jwks;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::build_context::BuildContext;
use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_handler_path::public_jwks_handler_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;
use crate::jwks_secret_store_path::server_secret_store_canonical_path;
use crate::jwks_verifier_path::public_jwks_verifier_canonical_path;
use crate::mint_access_token_handler_path::mint_access_token_handler_canonical_path;

pub(crate) fn jwks_pass(
    context: &mut BuildContext,
    bindings: &ContainerBindings,
    client_bindings: &[JwksClientBinding],
) {
    let server = JwksServerModule {
        has_handler: bindings.provides(&public_jwks_handler_canonical_path()),
        has_minter: bindings.provides(&mint_access_token_handler_canonical_path()),
        has_roller: bindings.provides(&jwks_roller_canonical_path()),
        has_secret_store: bindings.provides(&server_secret_store_canonical_path()),
    };

    let clients: Vec<JwksClientModule> = client_bindings
        .iter()
        .map(|binding| JwksClientModule {
            has_client: bindings.provides(&jwks_client_canonical_path(&binding.module_segment)),
            has_verifier: bindings.provides(&public_jwks_verifier_canonical_path(
                &binding.module_segment,
            )),
            segment: binding.module_segment.clone(),
        })
        .collect();

    context.set_framework_services(jwks_framework_services(bindings, client_bindings));

    let any_server =
        server.has_handler || server.has_minter || server.has_roller || server.has_secret_store;
    let any_client = !clients.is_empty();

    if any_server || any_client {
        context.extend_modules(render_jwks(&server, &clients));
        context.enable_jwks();
    }
}
