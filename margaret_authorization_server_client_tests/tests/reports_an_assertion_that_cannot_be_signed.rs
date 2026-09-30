use std::collections::BTreeSet;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::private_key_jwt_client::private_key_jwt_client;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn reports_an_assertion_that_cannot_be_signed() {
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

    let outcome = server
        .client(Arc::new(private_key_jwt_client(unrolled_store())))
        .client_credentials(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        })
        .await;

    server.stop().await;

    assert!(matches!(
        outcome,
        Err(AuthorizationServerClientError::AssertionSigning(
            JwksSecretStoreError::SecretUnavailable
        ))
    ));
}
