use crate::header_type::HeaderType;
use crate::key_id::KeyId;

pub struct VerifiedJws {
    pub kid: KeyId,
    pub payload: Vec<u8>,
    pub typ: Option<HeaderType>,
}
