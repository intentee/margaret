use margaret_handler_error::handler_error::HandlerError;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::failing_userinfo_claims::FailingUserinfoClaims;
use margaret_oidc_provider_tests::userinfo_handled::userinfo_handled;

#[tokio::test]
async fn reports_a_userinfo_claims_provider_that_fails() {
    let Err(HandlerError::Consumer { source }) =
        userinfo_handled(&END_USER_SUBJECT.to_string(), FailingUserinfoClaims).await
    else {
        panic!("a failing claims provider is reported");
    };

    assert!(matches!(
        source.downcast_ref::<ProviderError>(),
        Some(ProviderError::UserinfoClaimsProvider(failure)) if failure.to_string() == "the profile store is unreachable"
    ));
}
