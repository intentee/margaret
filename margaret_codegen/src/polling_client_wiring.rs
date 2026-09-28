use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

pub(crate) struct PollingClientWiring {
    pub(crate) client: CanonicalPath,
    pub(crate) client_dependencies: Vec<FrameworkDependency>,
    pub(crate) verifier: CanonicalPath,
    pub(crate) verifier_injection: FrameworkInjectionRole,
}

impl PollingClientWiring {
    pub(crate) fn providers(self) -> [FrameworkProvider; 2] {
        let Self {
            client,
            client_dependencies,
            verifier,
            verifier_injection,
        } = self;

        [
            FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: client_dependencies,
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
                injection: verifier_injection,
                provided: verifier,
            },
        ]
    }
}
