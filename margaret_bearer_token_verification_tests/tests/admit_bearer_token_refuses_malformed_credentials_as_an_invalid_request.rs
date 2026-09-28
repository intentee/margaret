use margaret_bearer_token_verification_tests::admission_answer::AdmissionAnswer;
use margaret_bearer_token_verification_tests::unready_verifier::unready_verifier;

#[tokio::test]
async fn admit_bearer_token_refuses_malformed_credentials_as_an_invalid_request() {
    let answer = AdmissionAnswer::request(unready_verifier(), "Bearer ab=c").await;

    assert_eq!(answer.status, 400);
    assert_eq!(
        answer.challenge.as_deref(),
        Some("Bearer error=\"invalid_request\"")
    );
}
