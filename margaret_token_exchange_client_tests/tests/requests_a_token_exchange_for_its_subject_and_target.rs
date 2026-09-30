use std::collections::BTreeSet;
use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::body_limit::BodyLimit;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client::token_exchange::TokenExchange;
use margaret_token_exchange_client_tests::workload_subject_token::workload_subject_token;

#[tokio::test]
async fn requests_a_token_exchange_for_its_subject_and_target() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(FormEchoHandler {
            limit: BodyLimit::new(1024),
            wrapping: EchoWrapping::AccessToken,
        }),
    )
    .await;
    let exchanged = TokenExchange::create(Arc::new(server.client(Arc::new(secret_basic_client()))))
        .exchange(
            &workload_subject_token(),
            &TokenTarget {
                audience: TargetAudience::Audience("artifact-store".to_string()),
                scopes: BTreeSet::new(),
            },
        )
        .await
        .expect("a secret basic client needs no assertion");

    server.stop().await;

    let ExchangedToken::Exchanged(token) = exchanged else {
        panic!("the subject token is exchanged");
    };
    let echoed: Value = serde_json::from_str(token.secret()).expect("the echo is json");

    assert_eq!(
        echoed["fields"],
        json!({
            "audience": "artifact-store",
            "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
            "requested_token_type": "urn:ietf:params:oauth:token-type:access_token",
            "subject_token": "workload.identity.token",
            "subject_token_type": "urn:ietf:params:oauth:token-type:id_token",
        })
    );
}
