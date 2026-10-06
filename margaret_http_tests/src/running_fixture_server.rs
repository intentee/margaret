use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ServerConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::route_entry::RouteEntry;
use margaret_http::server::Server;
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
        address: SocketAddr,
        transport: TransportConfig,
        upload_config: UploadConfig,
        route_entries: Vec<RouteEntry>,
    ) -> Self {
        let ServerRoutes {
            named_handlers,
            router,
        } = ServerRoutes::build(route_entries).expect("the route entries register cleanly");
        let server = Server::new(address.to_string(), transport, upload_config, router);
        let forward_targets = Arc::new(ForwardTargets::new(named_handlers));
        let bound = BoundServer::bind(Arc::new(server), forward_targets)
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
        Self::start_at(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            server_config,
            route_entries,
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start_at(
        ip: IpAddr,
        server_config: Arc<ServerConfig>,
        route_entries: Vec<RouteEntry>,
    ) -> Self {
        Self::serve(
            SocketAddr::new(ip, 0),
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
        Self::serve(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            TransportConfig::Plain,
            upload_config,
            route_entries,
        )
        .await
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
