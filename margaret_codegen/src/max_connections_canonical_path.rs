use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn max_connections_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "database".to_string(),
        "max_connections".to_string(),
        "MaxConnections".to_string(),
    ])
}
