use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_jwks_codegen::jwks_endpoint_tag::jwks_endpoint_tag;

use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_handler_path::public_jwks_handler_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;
use crate::jwks_secret_storage_path::jwks_secret_storage_canonical_path;
use crate::jwks_verifier_path::public_jwks_verifier_canonical_path;

pub(crate) fn jwks_framework_providers() -> Vec<FrameworkProvider> {
    let roller = jwks_roller_canonical_path();
    let client = jwks_client_canonical_path();

    vec![
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Provider(
                    jwks_secret_storage_canonical_path(),
                )],
                is_async: false,
                method: "create".to_string(),
            },
            enablement: FrameworkEnablement::Dependency,
            provided: roller.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "public_jwks_handler".to_string(),
                source: roller,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            provided: public_jwks_handler_canonical_path(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Endpoint(jwks_endpoint_tag())],
                is_async: false,
                method: "create".to_string(),
            },
            enablement: FrameworkEnablement::Always,
            provided: client.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "verifier".to_string(),
                source: client,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            provided: public_jwks_verifier_canonical_path(),
        },
    ]
}
