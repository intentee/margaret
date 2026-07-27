use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_client_module::JwksClientModule;
use margaret_jwks_codegen::jwks_server_module::JwksServerModule;
use margaret_jwks_codegen::render_jwks::render_jwks;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::jwks_artifacts::JwksArtifacts;
use crate::jwks_client_canonical_path::jwks_client_canonical_path;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::mint_access_token_handler_canonical_path::mint_access_token_handler_canonical_path;
use crate::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use crate::public_jwks_verifier_canonical_path::public_jwks_verifier_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

pub(crate) fn build_jwks_artifacts(
    bindings: &ContainerBindings,
    client_bindings: &[JwksClientBinding],
) -> JwksArtifacts {
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
    let services = jwks_framework_services(bindings, client_bindings);
    let any_server =
        server.has_handler || server.has_minter || server.has_roller || server.has_secret_store;
    let enabled = any_server || !clients.is_empty();
    let modules = if enabled {
        render_jwks(&server, &clients)
    } else {
        Vec::new()
    };

    JwksArtifacts {
        enabled,
        modules,
        services,
    }
}
