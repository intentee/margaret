use std::collections::BTreeMap;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::clears_the_transaction::clears_the_transaction;
use margaret_oidc_sign_in_tests::fixture_admission::FixtureAdmission;
use margaret_oidc_sign_in_tests::fixture_callback_handler::fixture_callback_handler;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn refuses_a_callback_without_its_transaction() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let started = started_with_sessions().await;
    let answered = fixture_callback_handler(
        fixture.flow.clone(),
        FixtureAdmission::Refusing,
        started.database.clone(),
    )
    .handle(&callback_request(
        "unrelated=cookie",
        &BTreeMap::from([("code", "SplxlOBeZQQYbYS6WxSbIA"), ("state", "forged")]),
    ))
    .await;

    fixture.server.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response))
            if response.status() == 403 && clears_the_transaction(&response)
    ));
}
