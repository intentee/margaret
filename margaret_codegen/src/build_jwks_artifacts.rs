use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_server_module::JwksServerModule;
use margaret_jwks_codegen::jwks_server_part::JwksServerPart;
use margaret_jwks_codegen::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use margaret_jwks_codegen::render_jwks::render_jwks;

use crate::framework_artifacts::FrameworkArtifacts;
use crate::jwks_framework_services::jwks_framework_services;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

pub(crate) fn build_jwks_artifacts(bindings: &ContainerBindings) -> FrameworkArtifacts {
    let mut server = JwksServerModule::default();

    server.enable_if(
        JwksServerPart::Handler,
        bindings.provides(&public_jwks_handler_canonical_path()),
    );
    server.enable_if(
        JwksServerPart::Roller,
        bindings.provides(&jwks_roller_canonical_path()),
    );
    server.enable_if(
        JwksServerPart::SecretStore,
        bindings.provides(&server_secret_store_canonical_path()),
    );

    let enabled = !server.is_empty();

    FrameworkArtifacts {
        enabled,
        modules: if enabled {
            vec![render_jwks(&server)]
        } else {
            Vec::new()
        },
        services: jwks_framework_services(bindings),
    }
}
