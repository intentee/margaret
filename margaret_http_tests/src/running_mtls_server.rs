use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use rustls::ServerConfig;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_http::body_limit::BodyLimit;
use margaret_http::bound_server::BoundServer;
use margaret_http::forward_targets::ForwardTargets;
use margaret_http::handler::Handler;
use margaret_http::request::Request;
use margaret_http::require_peer_spiffe_id::require_peer_spiffe_id;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::router_builder::RouterBuilder;
use margaret_http::server::Server;
use margaret_http::server_registry::ServerRegistry;
use margaret_http::transport_config::TransportConfig;
use margaret_http::upload_config::UploadConfig;

struct EchoPeer;

#[async_trait]
impl Handler for EchoPeer {
    async fn handle(&self, request: &Request) -> ResponseContinuation {
        match require_peer_spiffe_id(request) {
            Ok(spiffe_id) => ResponseContinuation::Done(Response::text(
                200,
                format!("{}{}", spiffe_id.trust_domain(), spiffe_id.path()),
            )),
            Err(response) => ResponseContinuation::Done(response),
        }
    }
}

pub struct RunningMtlsServer {
    address: SocketAddr,
    cancellation_token: CancellationToken,
    join_handle: JoinHandle<()>,
}

impl RunningMtlsServer {
    pub async fn start(server_config: Arc<ServerConfig>) -> Self {
        let server = Server::new(
            "mtls",
            "127.0.0.1:0".to_string(),
            "https://127.0.0.1",
            TransportConfig::MutualTls { server_config },
            UploadConfig::Disabled,
            BodyLimit::default(),
            RouterBuilder::empty()
                .route("GET", "/", Arc::new(EchoPeer))
                .build(),
        );
        let server_registry = Arc::new(ServerRegistry::new(vec![server]));
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));
        let bound = BoundServer::bind(server_registry, forward_targets, Arc::from("mtls"))
            .await
            .expect("the mTLS server binds");
        let address = bound.local_addr();
        let cancellation_token = CancellationToken::new();
        let join_handle = tokio::spawn(bound.serve(cancellation_token.clone()));

        Self {
            address,
            cancellation_token,
            join_handle,
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.join_handle
            .await
            .expect("the mTLS server task finishes cleanly");
    }
}
