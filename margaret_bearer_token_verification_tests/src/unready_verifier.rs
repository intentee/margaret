use std::sync::Arc;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;

#[must_use]
pub fn unready_verifier() -> BearerTokenVerifier {
    BearerTokenVerifier::new_for_access_tokens(
        Arc::new(fixture_trust()),
        VerificationKeySetHolder::default(),
    )
}
