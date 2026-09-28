use margaret_bearer_token_verification_tests::access_token_verifier_holding::access_token_verifier_holding;
use margaret_bearer_token_verification_tests::admission_answer::AdmissionAnswer;
use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jws_verification::key_set_parsing::KeySetParsing;

#[tokio::test]
async fn admit_bearer_token_refuses_a_rejected_token_as_an_invalid_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let KeySetParsing::Accepted(key_set) = published_key_set(&secret) else {
        panic!("the published key set is accepted");
    };

    let answer = AdmissionAnswer::request(
        access_token_verifier_holding(key_set),
        "Bearer opaque-token",
    )
    .await;

    assert_eq!(answer.status, 401);
    assert_eq!(
        answer.challenge.as_deref(),
        Some("Bearer error=\"invalid_token\"")
    );
}
