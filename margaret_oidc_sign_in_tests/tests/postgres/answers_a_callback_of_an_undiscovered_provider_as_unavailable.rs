use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::fixture_client_id::FIXTURE_CLIENT_ID;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::clears_the_transaction::clears_the_transaction;
use margaret_oidc_sign_in_tests::fixture_admission::FixtureAdmission;
use margaret_oidc_sign_in_tests::fixture_callback_handler::fixture_callback_handler;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn answers_a_callback_of_an_undiscovered_provider_as_unavailable() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let started = started_with_sessions().await;
    let SignInBeginning::Redirected(response) = fixture.flow.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);
    let undiscovered = fixture_sign_in_flow(
        Arc::new(AuthorizationServerClient::with_client_secret_basic(
            fixture.server.request_client(),
            Arc::new(IssuerMetadata::awaiting()),
            Arc::new(TrustedIssuer::polled(
                Arc::new(IssuerKeySet::awaiting()),
                localhost_trust(),
            )),
            FIXTURE_CLIENT_ID,
            fixture_client_secret(),
        )),
        Arc::clone(&fixture.secrets),
    );
    let answered = fixture_callback_handler(
        undiscovered,
        FixtureAdmission::Refusing,
        started.database.clone(),
    )
    .handle(&callback_request(
        &begun.cookie_pair(),
        &BTreeMap::from([
            ("code", "SplxlOBeZQQYbYS6WxSbIA"),
            ("state", begun.authorization_parameter("state")),
        ]),
    ))
    .await;

    fixture.server.stop().await;

    assert!(matches!(
        answered,
        Ok(ResponseContinuation::Done(response))
            if response.status() == 503 && clears_the_transaction(&response)
    ));
}
