use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn issuer_request_client_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "issuer_request".to_string(),
        "issuer_request_client".to_string(),
        "IssuerRequestClient".to_string(),
    ])
}
