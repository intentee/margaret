use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_cookie_jar::server_cookies::ServerCookies;
use margaret_http::body_limit::BodyLimit;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;
use margaret_service::server_service::ServerService;

#[tokio::test]
async fn binds_and_drains_on_cancellation() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let server_registry = Arc::new(ServerRegistry::new(vec![Server::new(
        "public",
        "127.0.0.1:0".to_string(),
        TransportConfig::Plain,
        UploadConfig::Disabled,
        BodyLimit::default(),
        Router::build(Vec::new()).expect("an empty router builds"),
        ServerCookies::Absent,
    )]));
    let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
    let service = ServerService::new(server_registry, forward_targets, "public");

    Box::new(service)
        .run(cancellation_token)
        .await
        .expect("the server binds and drains");
}
