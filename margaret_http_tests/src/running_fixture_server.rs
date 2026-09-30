use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ServerConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::route_entry::RouteEntry;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;
use margaret_http_uploaded_file::upload_config::UploadConfig;

pub struct RunningFixtureServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<()>,
}

impl RunningFixtureServer {
    async fn serve(
        transport: TransportConfig,
        upload_config: UploadConfig,
        route_entries: Vec<RouteEntry>,
    ) -> Self {
        let ServerRoutes {
            named_handlers,
            router,
        } = ServerRoutes::build(route_entries).expect("the route entries register cleanly");
        let server = Server::new(
            "fixture",
            "127.0.0.1:0".to_string(),
            transport,
            upload_config,
            router,
        );
        let server_registry = Arc::new(ServerRegistry::new(vec![server]));
        let forward_targets = Arc::new(ForwardTargets::new(named_handlers));
        let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("fixture"))
            .await
            .expect("the fixture server binds");
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

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start(server_config: Arc<ServerConfig>, route_entries: Vec<RouteEntry>) -> Self {
        Self::serve(
            TransportConfig::MutualTls { server_config },
            UploadConfig::Disabled,
            route_entries,
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start_plain(upload_config: UploadConfig, route_entries: Vec<RouteEntry>) -> Self {
        Self::serve(TransportConfig::Plain, upload_config, route_entries).await
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    #[must_use]
    pub fn port(&self) -> u16 {
        self.address.port()
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.join_handle
            .await
            .expect("the fixture server task finishes cleanly");
    }
}
