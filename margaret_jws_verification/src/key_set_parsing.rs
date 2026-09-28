use crate::key_set_rejection::KeySetRejection;
use crate::verification_key_set::VerificationKeySet;

pub enum KeySetParsing {
    Accepted(VerificationKeySet),
    Rejected(KeySetRejection),
}
