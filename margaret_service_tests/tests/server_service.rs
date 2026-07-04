use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::router_builder::RouterBuilder;
use margaret_http::server::Server;
use margaret_http::servers::Servers;
use margaret_http::upload_config::UploadConfig;
use margaret_service::server_service::ServerService;

#[tokio::test]
async fn binds_and_drains_on_cancellation() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let servers = Arc::new(Servers::new(
        vec![Server::new(
            "public",
            "127.0.0.1:0".to_string(),
            "http://127.0.0.1",
            UploadConfig::Disabled,
            RouterBuilder::empty().build(),
        )],
        Vec::new(),
    ));
    let service = ServerService::new(servers, "public");

    Box::new(service)
        .run(cancellation_token)
        .await
        .expect("the server binds and drains");
}
