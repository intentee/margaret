use std::collections::BTreeSet;
use std::sync::Arc;

use url::Url;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_oidc_sign_in::sign_in_error::SignInError;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_request::SignInRequest;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn propagates_a_transaction_that_cannot_be_signed() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body: Vec::new(),
            content_type: "application/json",
            status: 200,
        }),
    )
    .await;
    let flow = SignInFlow::create(
        Arc::new(server.client(Arc::new(secret_basic_client()))),
        Arc::new(unrolled_store()),
    );

    let beginning = flow
        .begin(SignInRequest {
            callback: Url::parse("https://client.example/callback").expect("the callback is a url"),
            scopes: BTreeSet::new(),
        })
        .await;

    server.stop().await;

    assert!(matches!(
        beginning,
        Err(SignInError::TransactionSecret(
            JwksSecretStoreError::SecretUnavailable
        ))
    ));
}
