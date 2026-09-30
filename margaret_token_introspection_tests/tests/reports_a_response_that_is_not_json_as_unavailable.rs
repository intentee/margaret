use margaret_http::response_continuation::ResponseContinuation;
use margaret_token_introspection::introspection_admission::IntrospectionAdmission;
use margaret_token_introspection_tests::introspected_with_body::introspected_with_body;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;

#[tokio::test]
async fn reports_a_response_that_is_not_json_as_unavailable() {
    assert!(matches!(
        introspected_with_body::<RepositoryClaims>(200, b"active".to_vec()).await,
        IntrospectionAdmission::Refused(ResponseContinuation::Done(response))
            if response.status() == 503
    ));
}
