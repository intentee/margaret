use margaret_bearer_token_verification_tests::admission_answer::AdmissionAnswer;
use margaret_bearer_token_verification_tests::unready_verifier::unready_verifier;

#[tokio::test]
async fn admit_bearer_token_refuses_while_the_signing_keys_are_unavailable() {
    let answer = AdmissionAnswer::request(unready_verifier(), "Bearer any.token.value").await;

    assert_eq!(answer.status, 503);
    assert_eq!(answer.challenge, None);
}
