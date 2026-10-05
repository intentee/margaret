use std::collections::BTreeSet;
use std::sync::Arc;

use serde_json::Value;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client::token_exchange::TokenExchange;

use crate::workload_subject_token::workload_subject_token;

/// # Panics
///
/// Panics when the exchange of a secret basic client fails to be sent.
pub async fn exchange_answered_with(status: u16, body: &Value) -> ExchangedToken {
    let server = FixtureAuthorizationServer::start(
        "/token",
        MethodHandler::head(
            RouteMethod::Post,
            Arc::new(StaticHandler {
                body: body.to_string().into_bytes(),
                content_type: "application/json",
                status,
            }),
        ),
    )
    .await;
    let exchanged =
        TokenExchange::create(Arc::new(server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))))
        .exchange(
            &workload_subject_token(),
            &TokenTarget {
                audience: TargetAudience::Unspecified,
                scopes: BTreeSet::new(),
            },
        )
        .await;

    server.stop().await;

    exchanged
}
