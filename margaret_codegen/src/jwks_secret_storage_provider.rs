use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_container::uri_selected_provider::UriSelectedProvider;

fn canonical_path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(
        segments
            .iter()
            .map(|segment| (*segment).to_string())
            .collect(),
    )
}

pub(crate) fn jwks_secret_storage_provider() -> FrameworkProvider {
    FrameworkProvider::UriSelected(UriSelectedProvider {
        argument_name: "jwks-secret-storage".to_string(),
        resolver: canonical_path(&[
            "margaret_jwks_secret_storage_selection",
            "resolve_jwks_secret_storage",
            "resolve_jwks_secret_storage",
        ]),
        trait_path: canonical_path(&[
            "margaret_jwks_roller",
            "jwks_secret_storage",
            "JwksSecretStorage",
        ]),
        value_type: canonical_path(&[
            "margaret_jwks_secret_storage_selection",
            "jwks_secret_storage_uri",
            "JwksSecretStorageUri",
        ]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provides_the_jwks_secret_storage_wiring_paths() {
        assert!(matches!(
            jwks_secret_storage_provider(),
            FrameworkProvider::UriSelected(provider)
                if provider.argument_name == "jwks-secret-storage"
                    && provider.trait_path
                        == canonical_path(&[
                            "margaret_jwks_roller",
                            "jwks_secret_storage",
                            "JwksSecretStorage",
                        ])
                    && provider.resolver
                        == canonical_path(&[
                            "margaret_jwks_secret_storage_selection",
                            "resolve_jwks_secret_storage",
                            "resolve_jwks_secret_storage",
                        ])
                    && provider.value_type
                        == canonical_path(&[
                            "margaret_jwks_secret_storage_selection",
                            "jwks_secret_storage_uri",
                            "JwksSecretStorageUri",
                        ])
        ));
    }
}
