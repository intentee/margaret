use std::sync::Arc;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_flow_error::SignInFlowError;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn rejects_a_callback_that_is_not_a_url() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let created = SignInFlow::create(
        Arc::new(fixture.server.client(secret_basic_authentication())),
        Arc::clone(&fixture.secrets),
        "/sign-in/callback".to_string(),
        &["profile"],
    );

    fixture.server.stop().await;

    assert!(matches!(
        created,
        Err(SignInFlowError::MalformedCallback { callback, .. }) if callback == "/sign-in/callback"
    ));
}
