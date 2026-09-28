use crate::ignored_key::IgnoredKey;
use crate::verification_key_set::VerificationKeySet;

pub struct AcceptedKeySetDocument {
    pub ignored_keys: Vec<IgnoredKey>,
    pub key_set: VerificationKeySet,
}
