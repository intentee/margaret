use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;

use crate::echo_upgrade_handler::EchoUpgradeHandler;

pub struct RunningEchoServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    handle: JoinHandle<()>,
}

impl RunningEchoServer {
    pub async fn start() -> Self {
        let registry = Arc::new(ServerRegistry::new(vec![Server::new(
            "test",
            "127.0.0.1:0".to_owned(),
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(vec![RouteEntry::new(
                "/ws",
                vec![MethodHandler::new("GET", Arc::new(EchoUpgradeHandler))],
            )])
            .expect("the websocket route registers cleanly"),
        )]));
        let bound = BoundServer::bind(
            registry,
            Arc::new(ForwardTargets::new(Vec::new())),
            Arc::from("test"),
        )
        .await
        .expect("the echo server binds");
        let address = bound.local_addr().expect("the bound server has an address");
        let cancellation_token = CancellationToken::new();
        let handle = tokio::spawn(bound.serve(cancellation_token.clone()));

        Self {
            address,
            cancellation_token,
            handle,
        }
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.handle.await.expect("the server task joins cleanly");
    }
}
