use crate::signing_keys_document::SigningKeysDocument;

#[derive(Clone)]
pub enum StoredSigningKeys {
    Absent,
    Stored(SigningKeysDocument),
}
