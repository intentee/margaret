use std::sync::Arc;

use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[must_use]
pub fn verifier_holding(key_set: VerificationKeySet) -> PublicJwksVerifier {
    let holder = VerificationKeySetHolder::default();

    holder.set(Some(Arc::new(key_set)));

    PublicJwksVerifier::new(Arc::new(fixture_trust()), holder)
}
