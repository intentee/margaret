use std::sync::Arc;

use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_reports_not_ready_before_the_first_poll() {
    let verifier = PublicJwksVerifier::new(
        Arc::new(fixture_trust()),
        VerificationKeySetHolder::default(),
    );

    assert!(matches!(
        verifier.verify::<TestClaims>("any.token.value", unix_time(1_700_000_000)),
        AccessTokenVerification::NotReady
    ));
}
