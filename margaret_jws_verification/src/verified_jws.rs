use crate::key_id::KeyId;

pub struct VerifiedJws<'jws> {
    pub kid: Option<&'jws KeyId>,
}
