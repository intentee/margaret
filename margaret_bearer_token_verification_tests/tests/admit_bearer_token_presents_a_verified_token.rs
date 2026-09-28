use margaret_bearer_token_verification_tests::access_token_verifier_holding::access_token_verifier_holding;
use margaret_bearer_token_verification_tests::admission_answer::AdmissionAnswer;
use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;

#[tokio::test]
async fn admit_bearer_token_presents_a_verified_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        published_key_set(&secret)
    else {
        panic!("the published key set is accepted");
    };

    let answer = AdmissionAnswer::request(
        access_token_verifier_holding(key_set),
        &format!("Bearer {token}"),
    )
    .await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.body, "subject");
}
