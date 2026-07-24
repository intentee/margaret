use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::framework_provider_construction::FrameworkProviderConstruction;

use crate::jwks_client_verifier_path::jwks_client_verifier_path;
use crate::jwks_server_publication_path::jwks_server_publication_path;

#[must_use]
pub fn identity_framework_providers() -> Vec<FrameworkProvider> {
    vec![
        FrameworkProvider {
            construction: FrameworkProviderConstruction::NewConstructor,
            path: jwks_server_publication_path(),
        },
        FrameworkProvider {
            construction: FrameworkProviderConstruction::NewConstructor,
            path: jwks_client_verifier_path(),
        },
    ]
}
