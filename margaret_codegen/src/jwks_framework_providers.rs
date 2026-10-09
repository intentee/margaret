use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_jwks_codegen::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;
use margaret_oidc_provider_codegen::declared_endpoint_routes::DeclaredEndpointRoutes;
use margaret_oidc_provider_codegen::provider_endpoint::ProviderEndpoint;

use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::jwks_secret_holder_canonical_path::jwks_secret_holder_canonical_path;
use crate::rsa_signing_keys_canonical_path::rsa_signing_keys_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;
use crate::signing_keys_tables_canonical_path::signing_keys_tables_canonical_path;

pub(crate) fn jwks_framework_providers(
    endpoint_routes: &DeclaredEndpointRoutes,
) -> Vec<FrameworkProvider> {
    let roller = jwks_roller_canonical_path();
    let jwks_secret_holder = jwks_secret_holder_canonical_path();
    let rsa_signing_keys = rsa_signing_keys_canonical_path();
    let server_secret_store = server_secret_store_canonical_path();
    let mut providers = vec![
        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Unmarked,
            provided: rsa_signing_keys.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Database {
                        framework_tables: vec![signing_keys_tables_canonical_path()],
                    },
                    FrameworkDependency::Provider(rsa_signing_keys),
                ],
                is_async: true,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Fallible,
            },
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Runner,
            provided: roller.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Accessor {
                accessor: "jwks_secret_holder".to_string(),
                source: roller.clone(),
            },
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::FrameworkState,
            provided: jwks_secret_holder.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(jwks_secret_holder),
                    FrameworkDependency::TokenIssuance,
                ],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: server_secret_store,
        },
    ];

    providers.extend(
        endpoint_routes
            .serves(ProviderEndpoint::Jwks)
            .then(|| FrameworkProvider {
                construction: FrameworkConstruction::Accessor {
                    accessor: "public_jwks_handler".to_string(),
                    source: roller,
                },
                enablement: FrameworkEnablement::Declared,
                injection: FrameworkInjectionRole::FrameworkState,
                provided: public_jwks_handler_canonical_path(),
            }),
    );

    providers
}
