use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn client_assertions_tables_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "client_assertions".to_string(),
        "margaret".to_string(),
        "tables".to_string(),
        "TABLES".to_string(),
    ])
}
