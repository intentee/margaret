use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_jwks_codegen::public_jwks_handler_canonical_path::public_jwks_handler_canonical_path;

use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::jwks_secret_storage_canonical_path::jwks_secret_storage_canonical_path;
use crate::mint_access_token_handler_canonical_path::mint_access_token_handler_canonical_path;
use crate::rsa_signing_keys_canonical_path::rsa_signing_keys_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

pub(crate) fn jwks_framework_providers() -> [FrameworkProvider; 5] {
    let roller = jwks_roller_canonical_path();
    let rsa_signing_keys = rsa_signing_keys_canonical_path();
    let server_secret_store = server_secret_store_canonical_path();

    [
        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Unmarked,
            provided: rsa_signing_keys.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(jwks_secret_storage_canonical_path()),
                    FrameworkDependency::Provider(rsa_signing_keys),
                ],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Infallible,
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
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: server_secret_store.clone(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![FrameworkDependency::Provider(server_secret_store)],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::WhenReferenced,
            injection: FrameworkInjectionRole::Unmarked,
            provided: mint_access_token_handler_canonical_path(),
        },
    ]
}
