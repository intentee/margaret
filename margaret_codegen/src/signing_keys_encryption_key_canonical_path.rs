use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) fn signing_keys_encryption_key_canonical_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "signing_keys_encryption".to_string(),
        "signing_keys_encryption_key".to_string(),
        "SigningKeysEncryptionKey".to_string(),
    ])
}
