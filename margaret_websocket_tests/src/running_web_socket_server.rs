use std::net::SocketAddr;
use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::route_entry::RouteEntry;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;
use margaret_http::web_socket_upgrade::WebSocketUpgrade;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_websocket::web_socket_upgrade_entry::WebSocketUpgradeEntry;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

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

    async fn start_from(
        upgrade: Arc<dyn WebSocketUpgrade>,
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        transport: TransportConfig,
    ) -> Self {
        let server = Server::new(
            "public",
            "127.0.0.1:0".to_string(),
            transport,
            UploadConfig::Disabled,
            ServerRoutes::build(vec![RouteEntry::web_socket("/ws", upgrade, middleware)])
                .expect("the route entries register cleanly")
                .router,
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

    pub async fn start_gated(middleware: Vec<Arc<dyn HttpMiddleware>>) -> Self {
        Self::start_from(
            Arc::new(WebSocketUpgradeEntry::new(
                TestSessionFactory,
                test_dispatch_table(),
            )),
            middleware,
            TransportConfig::Plain,
        )
        .await
    }

    pub async fn start_with<Factory>(factory: Factory) -> Self
    where
        Factory: WebSocketSessionFactory<Session = TestSession> + 'static,
    {
        Self::start_with_transport(factory, TransportConfig::Plain).await
    }

    pub async fn start_with_transport<Factory>(factory: Factory, transport: TransportConfig) -> Self
    where
        Factory: WebSocketSessionFactory<Session = TestSession> + 'static,
    {
        Self::start_from(
            Arc::new(WebSocketUpgradeEntry::new(factory, test_dispatch_table())),
            Vec::new(),
            transport,
        )
        .await
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.join_handle
            .await
            .expect("the websocket server task finishes cleanly");
    }
}
