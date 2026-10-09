use crate::signing_keys_revision::SigningKeysRevision;

#[derive(Clone)]
pub enum StoredSigningKeys {
    Absent,
    Stored(SigningKeysRevision),
}
