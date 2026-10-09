use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn signing_keys_store_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "signing_keys_database".to_string(),
        "database_signing_keys".to_string(),
        "DatabaseSigningKeys".to_string(),
    ])
}
