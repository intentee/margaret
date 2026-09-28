use margaret_bearer_token_verification_tests::admission_answer::AdmissionAnswer;
use margaret_bearer_token_verification_tests::unready_verifier::unready_verifier;

#[tokio::test]
async fn admit_bearer_token_admits_a_visitor_without_bearer_credentials_anonymously() {
    let answer = AdmissionAnswer::request(unready_verifier(), "Basic dXNlcjpwYXNz").await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.body, "anonymous");
}
