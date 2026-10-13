use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::verification_key::VerificationKey;

#[derive(Clone, Copy)]
pub(crate) struct KeyEntry<'secret> {
    pub(crate) public_jwk: &'secret Jwk,
    pub(crate) verification_key: &'secret VerificationKey,
}
