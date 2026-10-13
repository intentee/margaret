use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn rsa_signing_keys_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "jwks_keygen".to_string(),
        "generated_rsa_signing_keys".to_string(),
        "GeneratedRsaSigningKeys".to_string(),
    ])
}
