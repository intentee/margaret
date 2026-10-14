use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::container_error::ContainerError;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container_tests::render_with_framework_providers::render_with_framework_providers;

#[test]
fn rejects_a_framework_provider_whose_signing_keys_encryption_is_undeclared() {
    assert!(matches!(
        render_with_framework_providers(
            "fieldless",
            &[FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::UndeclaredSigningKeysEncryption],
                    is_async: true,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Fallible,
                },
                enablement: FrameworkEnablement::Declared,
                injection: FrameworkInjectionRole::Unmarked,
                provided: CanonicalPath::new(vec!["crate".to_string(), "KeyRoller".to_string()]),
            }],
        ),
        Err(ContainerError::MissingSigningKeysEncryption { provider })
            if provider == "crate::KeyRoller"
    ));
}
