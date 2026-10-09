use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn client_assertions_store_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "client_assertions_database".to_string(),
        "database_client_assertions".to_string(),
        "DatabaseClientAssertions".to_string(),
    ])
}
