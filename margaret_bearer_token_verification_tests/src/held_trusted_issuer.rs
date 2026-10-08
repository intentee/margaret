use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::held_key_set::held_key_set;

#[must_use]
pub fn held_trusted_issuer(trust: TokenTrust, key_set: VerificationKeySet) -> TrustedIssuer {
    TrustedIssuer::polled(held_key_set(key_set), trust)
}
