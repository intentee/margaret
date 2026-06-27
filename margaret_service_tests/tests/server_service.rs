use tokio_util::sync::CancellationToken;

use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_service::Service;
use margaret_service::server_service::ServerService;

#[tokio::test]
async fn binds_and_drains_on_cancellation() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let service = ServerService::new(Server::new(Router::empty()), "127.0.0.1:0".to_string());

    Box::new(service)
        .run(cancellation_token)
        .await
        .expect("the server binds and drains");
}
