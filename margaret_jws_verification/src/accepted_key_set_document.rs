use crate::disclosed_key::DisclosedKey;
use crate::ignored_key::IgnoredKey;
use crate::verification_key_set::VerificationKeySet;

pub struct AcceptedKeySetDocument {
    pub disclosed_keys: Vec<DisclosedKey>,
    pub ignored_keys: Vec<IgnoredKey>,
    pub key_set: VerificationKeySet,
}
