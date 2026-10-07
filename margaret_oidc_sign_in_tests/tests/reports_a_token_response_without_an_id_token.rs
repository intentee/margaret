use std::collections::BTreeMap;

use serde_json::json;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn reports_a_token_response_without_an_id_token() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(StaticHandler {
                body: json!({ "access_token": "2YotnFZFEjr1zCsicMWpAA", "token_type": "Bearer" })
                    .to_string()
                    .into_bytes(),
                content_type: "application/json",
                status: 200,
            })
            .is_ok()
    );

    let completion = fixture
        .flow
        .complete::<EmailClaims>(&callback_request(
            &begun.cookie_pair(),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("state", begun.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
