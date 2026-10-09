use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::unknown_userinfo_subject::UnknownUserinfoSubject;
use margaret_oidc_provider_tests::userinfo_handled::userinfo_handled;

#[tokio::test]
async fn answers_userinfo_for_an_unknown_subject_with_not_found() {
    assert!(matches!(
        userinfo_handled(&END_USER_SUBJECT.to_string(), UnknownUserinfoSubject).await,
        Ok(ResponseContinuation::Done(response)) if response.status() == 404
    ));
}
