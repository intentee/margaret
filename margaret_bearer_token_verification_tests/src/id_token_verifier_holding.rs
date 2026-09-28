use std::sync::Arc;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;

#[must_use]
pub fn id_token_verifier_holding(key_set: VerificationKeySet) -> BearerTokenVerifier {
    let holder = VerificationKeySetHolder::default();

    holder.set(Some(Arc::new(key_set)));

    BearerTokenVerifier::new_for_id_tokens(Arc::new(fixture_trust()), holder)
}
