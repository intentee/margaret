use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

pub(crate) fn polling_client_provider(
    client: CanonicalPath,
    dependencies: Vec<FrameworkDependency>,
    injection: FrameworkInjectionRole,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: "create".to_string(),
        },
        enablement: FrameworkEnablement::Always,
        injection,
        provided: client,
    }
}
