use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;

#[test]
fn public_jwks_verifier_reports_not_ready_before_the_first_poll() {
    let verifier = PublicJwksVerifier::new();

    assert!(!verifier.is_ready());

    let Err(error) = verifier.verify::<TestClaims>("any.token.value", test_instant(1_700_000_000))
    else {
        panic!("no token verifies before the jwks document is polled");
    };

    assert!(matches!(error, JwksClientError::NotReady));
    assert_eq!(
        error.to_string(),
        "the jwks document has not been fetched from the issuer yet"
    );
}
