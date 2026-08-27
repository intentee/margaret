use crate::jwks_key_error::JwksKeyError;
use crate::jws_algorithm::JwsAlgorithm;
use crate::signature_check::SignatureCheck;

pub(crate) trait ChecksSignature {
    fn algorithm(&self) -> JwsAlgorithm;

    /// # Errors
    ///
    /// Returns `JwksKeyError` propagated from the work it performs.
    fn check_signature(
        &self,
        signing_input: &str,
        signature_bytes: &[u8],
    ) -> Result<SignatureCheck, JwksKeyError>;
}
