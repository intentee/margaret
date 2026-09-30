use std::collections::BTreeMap;

use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_oidc_sign_in_tests::token_answer::TokenAnswer;

#[tokio::test]
async fn relays_a_refused_code_exchange() {
    let fixture = SignInFixture::start(secret_basic_client()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(TokenAnswer {
                body: json!({ "error": "invalid_grant" }),
                status: 400,
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
        .await
        .expect("the sign-in completes");

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Refused(SignInRefusal::TokenRefused(refusal)) if *refusal.error() == BasicErrorResponseType::InvalidGrant
    ));
}
