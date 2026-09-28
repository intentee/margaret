use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::jwks_client_canonical_path::jwks_client_canonical_path;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::jwks_secret_storage_canonical_path::jwks_secret_storage_canonical_path;
use crate::mint_access_token_handler_canonical_path::mint_access_token_handler_canonical_path;
use crate::polling_client_wiring::PollingClientWiring;
use crate::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use crate::public_jwks_verifier_canonical_path::public_jwks_verifier_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

fn client_providers(binding: &SegmentedTagBinding) -> [FrameworkProvider; 2] {
    PollingClientWiring {
        client: jwks_client_canonical_path(&binding.module_segment),
        client_dependencies: vec![
            FrameworkDependency::SingletonView(binding.declaring.clone()),
            FrameworkDependency::SingletonView(binding.declaring.clone()),
        ],
        verifier: public_jwks_verifier_canonical_path(&binding.module_segment),
        verifier_injection: FrameworkInjectionRole::JwksClientStore(binding.tag.clone()),
    }
    .providers()
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
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(roller),
                    FrameworkDependency::TokenIssuance,
                ],
                is_async: false,
                method: "create".to_string(),
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
    client_bindings: &[SegmentedTagBinding],
) -> Vec<FrameworkProvider> {
    let mut providers = Vec::from(server_providers());

    for binding in client_bindings {
        providers.extend(client_providers(binding));
    }

    providers
}
