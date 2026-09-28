use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_server_module::JwksServerModule;
use margaret_jwks_codegen::jwks_server_part::JwksServerPart;
use margaret_jwks_codegen::render_jwks::render_jwks;
use margaret_polling_client_codegen::polling_client_module::PollingClientModule;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::framework_artifacts::FrameworkArtifacts;
use crate::jwks_client_canonical_path::jwks_client_canonical_path;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::mint_access_token_handler_canonical_path::mint_access_token_handler_canonical_path;
use crate::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use crate::public_jwks_verifier_canonical_path::public_jwks_verifier_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

pub(crate) fn build_jwks_artifacts(
    bindings: &ContainerBindings,
    client_bindings: &[SegmentedTagBinding],
) -> FrameworkArtifacts {
    let mut server = JwksServerModule::default();

    server.enable_if(
        JwksServerPart::Handler,
        bindings.provides(&public_jwks_handler_canonical_path()),
    );
    server.enable_if(
        JwksServerPart::Minter,
        bindings.provides(&mint_access_token_handler_canonical_path()),
    );
    server.enable_if(
        JwksServerPart::Roller,
        bindings.provides(&jwks_roller_canonical_path()),
    );
    server.enable_if(
        JwksServerPart::SecretStore,
        bindings.provides(&server_secret_store_canonical_path()),
    );

    let clients: Vec<PollingClientModule> = client_bindings
        .iter()
        .map(|binding| PollingClientModule {
            has_client: bindings.provides(&jwks_client_canonical_path(&binding.module_segment)),
            has_verifier: bindings.provides(&public_jwks_verifier_canonical_path(
                &binding.module_segment,
            )),
            segment: binding.module_segment.clone(),
        })
        .collect();
    let services = jwks_framework_services(bindings, client_bindings);
    let enabled = !server.is_empty() || !clients.is_empty();
    let modules = if enabled {
        render_jwks(&server, &clients)
    } else {
        Vec::new()
    };

    FrameworkArtifacts {
        enabled,
        modules,
        services,
    }
}
