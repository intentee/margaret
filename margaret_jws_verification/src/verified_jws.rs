use crate::header_type::HeaderType;
use crate::key_id::KeyId;

pub struct VerifiedJws<'jws> {
    pub kid: &'jws KeyId,
    pub payload: &'jws [u8],
    pub typ: Option<&'jws HeaderType>,
}
