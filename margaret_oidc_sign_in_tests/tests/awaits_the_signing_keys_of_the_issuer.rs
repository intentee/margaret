use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_oidc_sign_in_tests::id_token_claims::id_token_claims;
use margaret_oidc_sign_in_tests::issued_token_answer::issued_token_answer;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn awaits_the_signing_keys_of_the_issuer() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let keyless = fixture_sign_in_flow(
        Arc::new(fixture.server.client(secret_basic_authentication())),
        Arc::clone(&fixture.roller),
    );
    let SignInBeginning::Redirected(response) = keyless.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(issued_token_answer(
                &fixture.issuer_secret.current().sign_json(
                    &id_token_claims(begun.authorization_parameter("nonce")),
                    JwtType::Jwt
                )
            ))
            .is_ok()
    );

    let completion = keyless
        .complete::<EmailClaims>(&callback_request(
            &begun.cookie_pair(),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("state", begun.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    assert!(matches!(completion, SignInCompletion::SigningKeysAwaited));
}
