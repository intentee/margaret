use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn request_body_type() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "http".to_string(),
        "bytes".to_string(),
        "Bytes".to_string(),
    ])
}
