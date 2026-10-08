use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn jwks_secret_holder_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "jwks_keygen".to_string(),
        "jwks_secret_holder".to_string(),
        "JwksSecretHolder".to_string(),
    ])
}
