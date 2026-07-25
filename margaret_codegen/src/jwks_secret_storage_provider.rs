use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;

use crate::jwks_secret_storage_path::jwks_secret_storage_canonical_path;

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
        construction: FrameworkConstruction::UriSelected {
            argument_name: "jwks-secret-storage".to_string(),
            resolver: canonical_path(&[
                "margaret", "framework", "jwks_secret_storage_selection",
                "resolve_jwks_secret_storage",
                "resolve_jwks_secret_storage",
            ]),
            value_type: canonical_path(&[
                "margaret", "framework", "jwks_secret_storage_selection",
                "jwks_secret_storage_uri",
                "JwksSecretStorageUri",
            ]),
        },
        enablement: FrameworkEnablement::WhenReferenced,
        injection: FrameworkInjectionRole::Unmarked,
        provided: jwks_secret_storage_canonical_path(),
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_path;
    use super::jwks_secret_storage_provider;
    use crate::jwks_secret_storage_path::jwks_secret_storage_canonical_path;
    use margaret_container::framework_construction::FrameworkConstruction;

    #[test]
    fn provides_the_jwks_secret_storage_wiring_paths() {
        let provider = jwks_secret_storage_provider();

        assert_eq!(provider.provided, jwks_secret_storage_canonical_path());
        assert!(matches!(
            provider.construction,
            FrameworkConstruction::UriSelected {
                argument_name,
                resolver,
                value_type,
            }
            if argument_name == "jwks-secret-storage"
                && resolver
                    == canonical_path(&[
                        "margaret", "framework", "jwks_secret_storage_selection",
                        "resolve_jwks_secret_storage",
                        "resolve_jwks_secret_storage",
                    ])
                && value_type
                    == canonical_path(&[
                        "margaret", "framework", "jwks_secret_storage_selection",
                        "jwks_secret_storage_uri",
                        "JwksSecretStorageUri",
                    ])
        ));
    }
}
