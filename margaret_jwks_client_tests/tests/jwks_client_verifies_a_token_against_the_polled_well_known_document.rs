use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwks_client_tests::polled_jwks_issuer::PolledJwksIssuer;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test(flavor = "multi_thread")]
async fn jwks_client_verifies_a_token_against_the_polled_well_known_document() {
    let issuer = PolledJwksIssuer::start().await;
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(issuer.secret.current());

    let BearerTokenVerification::Verified(polled) = issuer.verifier.verify::<TestClaims>(
        &RequestAuthorization::parse(Some(&format!("Bearer {token}"))),
        NumericDate::new(1_700_000_000),
    ) else {
        panic!("the polled document verifies the token");
    };

    assert_eq!(polled.claims, claims);

    issuer.stop().await;
}
