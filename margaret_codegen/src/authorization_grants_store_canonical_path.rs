use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn authorization_grants_store_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "authorization_grants_database".to_string(),
        "database_authorization_grants".to_string(),
        "DatabaseAuthorizationGrants".to_string(),
    ])
}
