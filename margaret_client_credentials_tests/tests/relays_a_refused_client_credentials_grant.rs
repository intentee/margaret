use std::sync::Arc;

use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn relays_a_refused_client_credentials_grant() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body: json!({ "error": "invalid_target" })
                .to_string()
                .into_bytes(),
            content_type: "application/json",
            status: 400,
        }),
    )
    .await;

    let acquired =
        ClientCredentials::create(Arc::new(server.client(Arc::new(secret_basic_client()))))
            .access_token(&artifact_store_target())
            .await
            .expect("a secret basic client needs no assertion");

    server.stop().await;

    assert!(matches!(
        acquired,
        AcquiredToken::Refused(refusal)
            if *refusal.error() == BasicErrorResponseType::Extension("invalid_target".to_string())
    ));
}
