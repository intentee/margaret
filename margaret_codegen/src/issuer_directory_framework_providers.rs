use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

use crate::issuer_directory_canonical_path::issuer_directory_canonical_path;
use crate::issuer_request_client_canonical_path::issuer_request_client_canonical_path;

pub(crate) fn issuer_directory_framework_providers(
    polled_key_sets: Vec<CanonicalPath>,
) -> Vec<FrameworkProvider> {
    if polled_key_sets.is_empty() {
        return Vec::new();
    }

    vec![
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: Vec::new(),
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Fallible,
            },
            enablement: FrameworkEnablement::Dependency,
            injection: FrameworkInjectionRole::Unmarked,
            provided: issuer_request_client_canonical_path(),
        },
        FrameworkProvider {
            construction: FrameworkConstruction::Constructor {
                dependencies: vec![
                    FrameworkDependency::Provider(issuer_request_client_canonical_path()),
                    FrameworkDependency::Providers(polled_key_sets),
                ],
                is_async: false,
                method: "create".to_string(),
                outcome: ConstructorOutcome::Infallible,
            },
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::Runner,
            provided: issuer_directory_canonical_path(),
        },
    ]
}
