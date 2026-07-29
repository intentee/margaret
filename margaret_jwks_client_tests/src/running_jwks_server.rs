use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ServerConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::handler::Handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_cancellation_cooperation::RequestCancellationCooperation;
use margaret_http::request_timeout::RequestTimeout;
use margaret_http::route_entry::RouteEntry;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

pub struct RunningJwksServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<()>,
}

impl RunningJwksServer {
    pub async fn start(server_config: Arc<ServerConfig>, handler: Arc<dyn Handler>) -> Self {
        let server = Server::new(
            "jwks",
            "127.0.0.1:0".to_string(),
            TransportConfig::MutualTls { server_config },
            UploadConfig::Disabled,
            BodyLimit::default(),
            RequestTimeout::default(),
            Router::build(vec![RouteEntry::new(
                WELL_KNOWN_JWKS_PATH,
                vec![MethodHandler::new(
                    "GET",
                    handler,
                    RequestCancellationCooperation::Immediate,
                )],
            )])
            .expect("the well known jwks route registers cleanly"),
        );
        let server_registry = Arc::new(ServerRegistry::new(vec![server]));
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
        let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("jwks"))
            .await
            .expect("the jwks server binds");
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
    pub fn port(&self) -> u16 {
        self.address.port()
    }

    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.join_handle
            .await
            .expect("the jwks server task finishes cleanly");
    }
}
