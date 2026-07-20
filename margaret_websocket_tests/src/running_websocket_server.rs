use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::route_entry::RouteEntry;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;
use margaret_websocket::web_socket_session_factory::WebSocketSessionFactory;
use margaret_websocket::web_socket_upgrade_entry::WebSocketUpgradeEntry;

use crate::test_dispatch_table::test_dispatch_table;
use crate::test_session::TestSession;
use crate::test_session_factory::TestSessionFactory;

pub struct RunningWebSocketServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<()>,
}

impl RunningWebSocketServer {
    pub async fn start() -> Self {
        Self::start_with(TestSessionFactory).await
    }

    pub async fn start_with<Factory>(factory: Factory) -> Self
    where
        Factory: WebSocketSessionFactory<Session = TestSession> + 'static,
    {
        let entry = Arc::new(WebSocketUpgradeEntry::new(factory, test_dispatch_table()));
        let server = Server::new(
            "public",
            "127.0.0.1:0".to_string(),
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(vec![RouteEntry::web_socket("/ws", entry)])
                .expect("the route entries register cleanly"),
        );
        let server_registry = Arc::new(ServerRegistry::new(vec![server]));
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
        let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("public"))
            .await
            .expect("the websocket server binds");
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
            .expect("the websocket server task finishes cleanly");
    }
}
