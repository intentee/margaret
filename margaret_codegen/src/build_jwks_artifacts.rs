use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_server_module::JwksServerModule;
use margaret_jwks_codegen::jwks_server_part::JwksServerPart;
use margaret_jwks_codegen::render_jwks::render_jwks;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::framework_artifacts::FrameworkArtifacts;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::mint_access_token_handler_canonical_path::mint_access_token_handler_canonical_path;
use crate::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
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

    let segments: Vec<&str> = client_bindings
        .iter()
        .map(|binding| binding.module_segment.as_str())
        .collect();
    let services = jwks_framework_services(bindings, client_bindings);
    let enabled = !server.is_empty() || !segments.is_empty();
    let modules = if enabled {
        render_jwks(&server, &segments)
    } else {
        Vec::new()
    };

    FrameworkArtifacts {
        enabled,
        modules,
        services,
    }
}
