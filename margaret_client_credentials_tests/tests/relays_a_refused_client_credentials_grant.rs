use std::sync::Arc;

use oauth2::basic::BasicErrorResponseType;
use serde_json::json;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn relays_a_refused_client_credentials_grant() {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: json!({ "error": "invalid_target" })
                    .to_string()
                    .into_bytes(),
                content_type: "application/json",
                status: 400,
            }),
        ),
    )
    .await;

    let acquired =
        ClientCredentials::create(Arc::new(server.client(secret_basic_authentication())))
            .access_token(&artifact_store_target())
            .await;

    server.stop().await;

    assert!(matches!(
        acquired,
        AcquiredToken::Refused(refusal)
            if *refusal.error() == BasicErrorResponseType::Extension("invalid_target".to_string())
    ));
}
