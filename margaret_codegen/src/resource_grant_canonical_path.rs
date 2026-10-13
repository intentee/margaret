use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn resource_grant_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "client_credentials".to_string(),
        "resource_grant".to_string(),
        "ResourceGrant".to_string(),
    ])
}
