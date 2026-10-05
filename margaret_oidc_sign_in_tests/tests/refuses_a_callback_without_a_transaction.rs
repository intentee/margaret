use http::Method;

use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn refuses_a_callback_without_a_transaction() {
    let fixture = SignInFixture::start(secret_basic_client()).await;

    let completion = fixture
        .flow
        .complete::<EmailClaims>(
            &FixtureRequest::new(
                Method::GET,
                "/callback?code=SplxlOBeZQQYbYS6WxSbIA&state=any",
            )
            .into_request(),
        )
        .await;

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Refused(SignInRefusal::TransactionMissing)
    ));
}
