use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;
use margaret_jwks_client_tests::test_instant::test_instant;

#[test]
fn public_jwks_verifier_reports_not_ready_before_the_first_poll() {
    let verifier = PublicJwksVerifier::new(test_expected_claims(), PublicJwksHolder::default());

    let verification = verifier
        .verify::<TestClaims>("any.token.value", test_instant(1_700_000_000))
        .expect("an unpolled verifier is not a system failure");

    assert!(matches!(verification, AccessTokenVerification::NotReady));
}
