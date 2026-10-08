use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn database_url_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "database".to_string(),
        "database_url".to_string(),
        "DatabaseUrl".to_string(),
    ])
}
