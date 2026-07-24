use margaret_attributes::canonical_path::CanonicalPath;

#[must_use]
pub fn jwks_server_publication_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_jwks_roller_server".to_string(),
        "jwks_publication".to_string(),
        "JwksPublication".to_string(),
    ])
}
