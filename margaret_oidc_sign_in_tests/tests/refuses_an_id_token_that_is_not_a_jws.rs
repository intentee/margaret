use std::collections::BTreeMap;

use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::issued_token_answer::issued_token_answer;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn refuses_an_id_token_that_is_not_a_jws() {
    let fixture = SignInFixture::start(secret_basic_client()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(issued_token_answer("not-a-jws"))
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
        SignInCompletion::Refused(SignInRefusal::IdTokenRejected(JwtRejection::Jws(_)))
    ));
}
