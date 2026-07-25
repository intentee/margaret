use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_handler_path::public_jwks_handler_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;
use crate::jwks_secret_storage_path::jwks_secret_storage_canonical_path;
use crate::jwks_secret_store_path::server_secret_store_canonical_path;
use crate::jwks_verifier_path::public_jwks_verifier_canonical_path;
use crate::mint_access_token_handler_path::mint_access_token_handler_canonical_path;

fn client_providers(binding: &JwksClientBinding) -> [FrameworkProvider; 2] {
    let client = jwks_client_canonical_path(&binding.tag);

    [
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Endpoint(binding.endpoint.clone())],
                is_async: false,
                method: "create".to_string(),
            },
            enablement: FrameworkEnablement::Always,
            injection: FrameworkInjectionRole::Unmarked,
            provided: client.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "verifier".to_string(),
                source: client,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::JwksClientStore(binding.tag.clone()),
            provided: public_jwks_verifier_canonical_path(&binding.tag),
        },
    ]
}

fn server_providers() -> [FrameworkProvider; 4] {
    let roller = jwks_roller_canonical_path();
    let server_secret_store = server_secret_store_canonical_path();

    [
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Provider(
                    jwks_secret_storage_canonical_path(),
                )],
                is_async: false,
                method: "create".to_string(),
            },
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Unmarked,
            provided: roller.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "public_jwks_handler".to_string(),
                source: roller.clone(),
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: public_jwks_handler_canonical_path(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "server_secret_store".to_string(),
                source: roller,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::JwksServerStore,
            provided: server_secret_store.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Provider(server_secret_store)],
                is_async: false,
                method: "create".to_string(),
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: mint_access_token_handler_canonical_path(),
        },
    ]
}

pub(crate) fn jwks_framework_providers(
    client_bindings: &[JwksClientBinding],
) -> Vec<FrameworkProvider> {
    let mut providers = Vec::from(server_providers());

    for binding in client_bindings {
        providers.extend(client_providers(binding));
    }

    providers
}
