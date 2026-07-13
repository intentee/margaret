use std::convert::Infallible;
use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http::request::Parts;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use hyper_util::server::graceful::GracefulShutdown;
use hyper_util::server::graceful::Watcher;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::server::TlsStream;
use tokio_util::sync::CancellationToken;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::body_limit::BodyLimit;
use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::request_error::RequestError;
use crate::request_inputs::RequestInputs;
use crate::response::Response;
use crate::router::Router;
use crate::server_registry::ServerRegistry;
use crate::transport_config::TransportConfig;
use crate::upload_config::UploadConfig;

#[derive(Clone)]
enum BoundTransport {
    Plain,
    MutualTls { acceptor: TlsAcceptor },
}

#[derive(Clone)]
struct ConnectionContext {
    body_limit: BodyLimit,
    forward_targets: Arc<ForwardTargets>,
    peer_identity: Arc<PeerIdentity>,
    remote_addr: SocketAddr,
    router: Arc<Router>,
    upload_config: Arc<UploadConfig>,
}

pub struct BoundServer {
    body_limit: BodyLimit,
    forward_targets: Arc<ForwardTargets>,
    listener: TcpListener,
    router: Arc<Router>,
    transport: BoundTransport,
    upload_config: Arc<UploadConfig>,
}

impl BoundServer {
    pub async fn bind(
        server_registry: Arc<ServerRegistry>,
        forward_targets: Arc<ForwardTargets>,
        name: Arc<str>,
    ) -> std::io::Result<Self> {
        let Some(server) = server_registry.server(&name) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "the server is not registered",
            ));
        };
        let body_limit = server.body_limit();
        let router = server.router().clone();
        let transport = match server.transport().as_ref() {
            TransportConfig::Plain => BoundTransport::Plain,
            TransportConfig::MutualTls { server_config } => BoundTransport::MutualTls {
                acceptor: TlsAcceptor::from(server_config.clone()),
            },
        };
        let upload_config = server.upload_config().clone();
        let listener = TcpListener::bind(server.address()).await?;

        Ok(Self {
            body_limit,
            forward_targets,
            listener,
            router,
            transport,
            upload_config,
        })
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.listener
            .local_addr()
            .expect("a bound listener has a local address")
    }

    pub async fn serve(self, cancellation_token: CancellationToken) {
        let builder = Arc::new(Builder::new(TokioExecutor::new()));
        let graceful = GracefulShutdown::new();

        while let Some(accepted) = cancellation_token
            .run_until_cancelled(self.listener.accept())
            .await
        {
            accept_outcome(accepted)
                .map(|connection| self.spawn_connection(&builder, &graceful, connection));
        }

        graceful.shutdown().await;
    }

    fn spawn_connection(
        &self,
        builder: &Arc<Builder<TokioExecutor>>,
        graceful: &GracefulShutdown,
        AcceptedConnection {
            remote_addr,
            stream,
        }: AcceptedConnection,
    ) -> JoinHandle<()> {
        let builder = builder.clone();
        let watcher = graceful.watcher();
        let transport = self.transport.clone();
        let router = self.router.clone();
        let upload_config = self.upload_config.clone();
        let forward_targets = self.forward_targets.clone();
        let body_limit = self.body_limit;

        tokio::spawn(async move {
            match transport {
                BoundTransport::Plain => {
                    serve_connection(
                        builder,
                        watcher,
                        TokioIo::new(stream),
                        ConnectionContext {
                            body_limit,
                            forward_targets,
                            peer_identity: Arc::new(PeerIdentity::from_peer_certificate(None)),
                            remote_addr,
                            router,
                            upload_config,
                        },
                    )
                    .await;
                }
                BoundTransport::MutualTls { acceptor } => {
                    let tls_stream = match acceptor.accept(stream).await {
                        Ok(tls_stream) => tls_stream,
                        Err(error) => {
                            eprintln!("margaret_http: tls handshake error: {error}");

                            return;
                        }
                    };
                    let peer_identity = Arc::new(peer_identity_from_tls_stream(&tls_stream));

                    serve_connection(
                        builder,
                        watcher,
                        TokioIo::new(tls_stream),
                        ConnectionContext {
                            body_limit,
                            forward_targets,
                            peer_identity,
                            remote_addr,
                            router,
                            upload_config,
                        },
                    )
                    .await;
                }
            }
        })
    }
}

struct AcceptedConnection {
    remote_addr: SocketAddr,
    stream: TcpStream,
}

fn accept_outcome(
    accepted: std::io::Result<(TcpStream, SocketAddr)>,
) -> Option<AcceptedConnection> {
    match accepted {
        Ok((stream, remote_addr)) => Some(AcceptedConnection {
            remote_addr,
            stream,
        }),
        Err(error) => {
            eprintln!("margaret_http: accept error: {error}");

            None
        }
    }
}

fn peer_identity_from_tls_stream(tls_stream: &TlsStream<TcpStream>) -> PeerIdentity {
    PeerIdentity::from_peer_certificate(
        tls_stream
            .get_ref()
            .1
            .peer_certificates()
            .and_then(|certificates| certificates.first())
            .map(|certificate| certificate.as_ref()),
    )
}

fn report_connection_outcome<Error: Display>(outcome: Result<(), Error>) {
    if let Err(error) = outcome {
        eprintln!("margaret_http: connection error: {error}");
    }
}

fn error_response(error: RequestError) -> Response {
    match error {
        RequestError::PayloadTooLarge { .. } => Response::text(413, "Payload Too Large"),
        _ => Response::text(400, "Bad Request"),
    }
}

async fn serve_connection<Io>(
    builder: Arc<Builder<TokioExecutor>>,
    watcher: Watcher,
    io: Io,
    connection_context: ConnectionContext,
) where
    Io: hyper::rt::Read + hyper::rt::Write + Unpin + Send + 'static,
{
    let service = TowerToHyperService::new(tower::service_fn(
        move |request: http::Request<Incoming>| {
            let connection_context = connection_context.clone();

            async move { Ok::<_, Infallible>(dispatch(connection_context, request).await) }
        },
    ));

    report_connection_outcome(watcher.watch(builder.serve_connection(io, service)).await);
}

async fn dispatch(
    ConnectionContext {
        body_limit,
        forward_targets,
        peer_identity,
        remote_addr,
        router,
        upload_config,
    }: ConnectionContext,
    request: http::Request<Incoming>,
) -> http::Response<Full<Bytes>> {
    let (parts, incoming) = request.into_parts();
    let Parts {
        method,
        headers,
        uri,
        ..
    } = parts;
    let body = incoming.map_err(std::io::Error::other).boxed_unsync();

    match RequestInputs::parse(
        method,
        &uri,
        headers,
        remote_addr,
        body,
        &body_limit,
        &upload_config,
    )
    .await
    {
        Ok(inputs) => router
            .respond(
                Request::from_inputs(inputs).with_peer_identity(peer_identity),
                &forward_targets,
            )
            .await
            .into_http(),
        Err(error) => error_response(error).into_http(),
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Arc;

    use async_trait::async_trait;
    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tokio_util::sync::CancellationToken;

    use super::BoundServer;
    use super::accept_outcome;
    use super::error_response;
    use crate::body_limit::BodyLimit;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::request::Request;
    use crate::request_error::RequestError;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::router_builder::RouterBuilder;
    use crate::server::Server;
    use crate::server_registry::ServerRegistry;
    use crate::transport_config::TransportConfig;
    use crate::upload_config::UploadConfig;

    struct PlainOk;

    #[async_trait]
    impl Handler for PlainOk {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(200, "ok"))
        }
    }

    fn registry_with_one() -> Arc<ServerRegistry> {
        Arc::new(ServerRegistry::new(vec![Server::new(
            "test",
            "127.0.0.1:0".to_string(),
            "http://127.0.0.1",
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            RouterBuilder::empty()
                .route("GET", "/", Arc::new(PlainOk))
                .build(),
        )]))
    }

    fn empty_forward_targets() -> Arc<ForwardTargets> {
        Arc::new(ForwardTargets::new(Vec::new()))
    }

    #[test]
    fn maps_request_errors_to_status_codes() {
        assert_eq!(
            error_response(RequestError::PayloadTooLarge { limit: 8 })
                .into_http()
                .status()
                .as_u16(),
            413
        );
        assert_eq!(
            error_response(RequestError::MissingMultipartBoundary)
                .into_http()
                .status()
                .as_u16(),
            400
        );
    }

    #[test]
    fn skips_a_failed_accept() {
        assert!(
            accept_outcome(Err(std::io::Error::from(
                std::io::ErrorKind::ConnectionAborted
            )))
            .is_none()
        );
    }

    #[tokio::test]
    async fn fails_to_bind_an_unregistered_server() {
        assert!(
            BoundServer::bind(
                registry_with_one(),
                empty_forward_targets(),
                Arc::from("missing")
            )
            .await
            .is_err()
        );
    }

    #[tokio::test]
    async fn fails_to_bind_an_invalid_address() {
        let server_registry = Arc::new(ServerRegistry::new(vec![Server::new(
            "test",
            "this is not an address".to_string(),
            "http://127.0.0.1",
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            RouterBuilder::empty().build(),
        )]));

        assert!(
            BoundServer::bind(server_registry, empty_forward_targets(), Arc::from("test"))
                .await
                .is_err()
        );
    }

    async fn exchange(address: SocketAddr, request: &[u8], close_write: bool) -> String {
        let mut stream = TcpStream::connect(address)
            .await
            .expect("the client connects to the bound server");

        stream
            .write_all(request)
            .await
            .expect("the request reaches the server");

        if close_write {
            stream.shutdown().await.expect("the write half closes");
        }

        let mut response = Vec::new();

        stream
            .read_to_end(&mut response)
            .await
            .expect("the response is read to completion");

        String::from_utf8_lossy(&response).into_owned()
    }

    #[tokio::test]
    async fn serves_connections_until_cancellation() {
        let bound = BoundServer::bind(
            registry_with_one(),
            empty_forward_targets(),
            Arc::from("test"),
        )
        .await
        .expect("the server binds to an ephemeral port");
        let address = bound.local_addr();
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let (served, interrupted, unmatched, unsupported, truncated) = tokio::join!(
            exchange(
                address,
                b"GET / HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
                false,
            ),
            exchange(address, b"GET /interrupted HTTP/1.1\r\nHost: test\r\n", true),
            exchange(
                address,
                b"GET /missing HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
                false,
            ),
            exchange(
                address,
                b"OPTIONS / HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
                false,
            ),
            exchange(
                address,
                b"POST / HTTP/1.1\r\nHost: test\r\nContent-Type: application/json\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort",
                true,
            ),
        );

        assert!(served.contains(" 200 "));
        assert!(!interrupted.contains(" 200 "));
        assert!(unmatched.contains(" 404 "));
        assert!(unsupported.contains(" 405 "));
        assert!(truncated.contains(" 400 "));

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }
}
