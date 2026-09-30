use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection::introspection_admission::IntrospectionAdmission;

/// # Panics
///
/// Panics when the introspection of a secret basic client fails to be sent.
pub async fn introspected_with_body<TClaims: DeserializeOwned>(
    status: u16,
    body: Vec<u8>,
) -> IntrospectionAdmission<TClaims> {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/introspect",
        Arc::new(StaticHandler {
            body,
            content_type: "application/json",
            status,
        }),
    )
    .await;
    let admission = introspect_bearer_token(
        &RequestAuthorization::parse(Some("Bearer opaque-token")),
        &server.client(Arc::new(secret_basic_client())),
    )
    .await
    .expect("a secret basic client needs no assertion");

    server.stop().await;

    admission
}
