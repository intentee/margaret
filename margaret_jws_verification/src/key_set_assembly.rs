use crate::duplicate_key_id::DuplicateKeyId;
use crate::verification_key_set::VerificationKeySet;

pub enum KeySetAssembly {
    Assembled(VerificationKeySet),
    DuplicateKeyId(DuplicateKeyId),
}
