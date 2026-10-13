use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider_tests::fixed_userinfo_claims::FixedUserinfoClaims;
use margaret_oidc_provider_tests::userinfo_handled::userinfo_handled;

#[tokio::test]
async fn refuses_userinfo_for_a_token_of_a_client_itself() {
    assert!(matches!(
        userinfo_handled("portal", FixedUserinfoClaims { claims: "unread" }).await,
        Ok(ResponseContinuation::Done(response)) if response.status() == 401
    ));
}
