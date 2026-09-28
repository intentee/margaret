use margaret_jose_parameters::curve::Curve;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::verifier_holding::verifier_holding;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn public_jwks_verifier_accepts_a_freshly_signed_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(secret.current());

    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    let AccessTokenVerification::Verified(verified) =
        verifier_holding(key_set).verify::<TestClaims>(&token, unix_time(1_700_000_000))
    else {
        panic!("the freshly signed token verifies");
    };

    assert_eq!(verified.claims, claims);
}
