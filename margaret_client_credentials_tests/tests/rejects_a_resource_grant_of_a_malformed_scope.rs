use std::sync::Arc;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_client_credentials::client_credentials_error::ClientCredentialsError;
use margaret_client_credentials::resource_credentials::ResourceCredentials;
use margaret_client_credentials::resource_grant::ResourceGrant;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn rejects_a_resource_grant_of_a_malformed_scope() {
    let server = FixtureAuthorizationServer::start(
        "/unused",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "text/plain",
                status: 404,
            }),
        ),
    )
    .await;
    let created = ResourceCredentials::create(
        Arc::new(server.client(secret_basic_authentication())),
        ResourceGrant {
            audience: "attachments",
            scopes: &["files read"],
        },
    );

    server.stop().await;

    assert!(matches!(
        created,
        Err(ClientCredentialsError::MalformedGrantScope {
            rejection: ScopeRejection::Character,
            scope: "files read",
        })
    ));
}
