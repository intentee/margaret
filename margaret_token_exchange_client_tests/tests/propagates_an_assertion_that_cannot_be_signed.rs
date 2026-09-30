use std::collections::BTreeSet;
use std::sync::Arc;

use serde_json::json;

use margaret_authorization_server_client::authorization_server_client_error::AuthorizationServerClientError;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_exchange_client::token_exchange::TokenExchange;
use margaret_token_exchange_client_tests::workload_subject_token::workload_subject_token;

#[tokio::test]
async fn propagates_an_assertion_that_cannot_be_signed() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(StaticHandler {
            body: json!({}).to_string().into_bytes(),
            content_type: "application/json",
            status: 200,
        }),
    )
    .await;

    let exchanged = TokenExchange::create(Arc::new(
        server.client(Arc::new(OAuthClient {
            authentication: ClientAuthentication::PrivateKeyJwt(Arc::new(unrolled_store())),
            client_id: "client"
                .parse()
                .expect("the fixture client identifier is visible"),
        })),
    ))
    .exchange(
        &workload_subject_token(),
        &TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        },
    )
    .await;

    server.stop().await;

    assert!(matches!(
        exchanged,
        Err(AuthorizationServerClientError::AssertionSigning(_))
    ));
}
