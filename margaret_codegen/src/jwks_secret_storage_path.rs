use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn jwks_secret_storage_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_jwks_roller".to_string(),
        "jwks_secret_storage".to_string(),
        "JwksSecretStorage".to_string(),
    ])
}
