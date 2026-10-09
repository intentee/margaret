use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn issuer_directory_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "issuer_directory".to_string(),
        "issuer_directory".to_string(),
        "IssuerDirectory".to_string(),
    ])
}
