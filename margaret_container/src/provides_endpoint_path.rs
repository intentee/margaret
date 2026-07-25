use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn provides_endpoint_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_jwks_endpoint".to_string(),
        "provides_endpoint".to_string(),
        "ProvidesEndpoint".to_string(),
    ])
}
