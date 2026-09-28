use crate::key_id::KeyId;

pub struct VerifiedJws {
    pub kid: KeyId,
    pub payload: Vec<u8>,
}
