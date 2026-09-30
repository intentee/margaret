use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_error::SignInError;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn propagates_a_transaction_that_cannot_be_verified() {
    let fixture = SignInFixture::start(secret_basic_client()).await;
    let SignInBeginning::Redirected(response) = begin_sign_in(&fixture.flow).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);
    let unrolled = SignInFlow::create(
        Arc::new(fixture.server.client(Arc::new(secret_basic_client()))),
        Arc::new(unrolled_store()),
    );

    let completion = unrolled
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
        Err(SignInError::TransactionSecret(
            JwksSecretStoreError::SecretUnavailable
        ))
    ));
}
