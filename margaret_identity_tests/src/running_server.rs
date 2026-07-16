use std::net::SocketAddr;
use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub struct RunningServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<()>,
}

impl RunningServer {
    pub async fn start(server_routes: ServerRoutes, transport: TransportConfig) -> Self {
        let server = Server::new(
            "test",
            "127.0.0.1:0".to_string(),
            transport,
            UploadConfig::Disabled,
            BodyLimit::default(),
            server_routes.router,
        );
        let server_registry = Arc::new(ServerRegistry::new(vec![server]));
        let forward_targets = Arc::new(ForwardTargets::new(server_routes.named_handlers));
        let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("test"))
            .await
            .expect("the test server binds");
        let address = bound
            .local_addr()
            .expect("the bound listener reports its address");
        let cancellation_token = CancellationToken::new();
        let join_handle = tokio::spawn(bound.serve(cancellation_token.clone()));

        Self {
            address,
            cancellation_token,
            join_handle,
        }
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.join_handle
            .await
            .expect("the test server task finishes cleanly");
    }
}
