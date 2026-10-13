use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn token_target_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "authorization_server_client".to_string(),
        "token_target".to_string(),
        "TokenTarget".to_string(),
    ])
}
