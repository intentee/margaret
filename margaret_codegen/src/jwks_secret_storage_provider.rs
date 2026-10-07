use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_environment_variable_codegen::framework_environment_variable::FrameworkEnvironmentVariable;

use crate::jwks_secret_storage_canonical_path::jwks_secret_storage_canonical_path;

fn canonical_path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(
        segments
            .iter()
            .map(|segment| (*segment).to_string())
            .collect(),
    )
}

pub(crate) fn jwks_secret_storage_provider() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Resolved {
            dependencies: vec![FrameworkDependency::EnvironmentVariable {
                name: EnvironmentVariableName::from(
                    FrameworkEnvironmentVariable::JwksSecretStorage,
                ),
                value_type: canonical_path(&[
                    "margaret",
                    "framework",
                    "jwks_secret_storage_selection",
                    "jwks_secret_storage_uri",
                    "JwksSecretStorageUri",
                ]),
            }],
            resolver: canonical_path(&[
                "margaret",
                "framework",
                "jwks_secret_storage_selection",
                "resolve_jwks_secret_storage",
                "resolve_jwks_secret_storage",
            ]),
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: jwks_secret_storage_canonical_path(),
    }
}

#[cfg(test)]
mod tests {
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_dependency::FrameworkDependency;

    use super::canonical_path;
    use super::jwks_secret_storage_provider;
    use crate::jwks_secret_storage_canonical_path::jwks_secret_storage_canonical_path;

    #[test]
    fn resolves_the_jwks_secret_storage_from_its_environment_variable() {
        let provider = jwks_secret_storage_provider();

        assert_eq!(provider.provided, jwks_secret_storage_canonical_path());
        assert!(matches!(
            provider.construction,
            FrameworkConstruction::Resolved { dependencies, resolver }
                if resolver
                    == canonical_path(&[
                        "margaret",
                        "framework",
                        "jwks_secret_storage_selection",
                        "resolve_jwks_secret_storage",
                        "resolve_jwks_secret_storage",
                    ])
                    && matches!(
                        dependencies.as_slice(),
                        [FrameworkDependency::EnvironmentVariable { name, value_type }]
                            if name.as_str() == "MARGARET_JWKS_SECRET_STORAGE"
                                && *value_type
                                    == canonical_path(&[
                                        "margaret",
                                        "framework",
                                        "jwks_secret_storage_selection",
                                        "jwks_secret_storage_uri",
                                        "JwksSecretStorageUri",
                                    ])
                    )
        ));
    }
}
