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
use tokio_util::task::TaskTracker;

use margaret_peer_identity::peer_identity::PeerIdentity;

use crate::body_limit::BodyLimit;
use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::request_error::RequestError;
use crate::request_inputs::RequestInputs;
use crate::response::Response;
use crate::router::Router;
use crate::served_outcome::ServedOutcome;
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
    cancellation_token: CancellationToken,
    forward_targets: Arc<ForwardTargets>,
    peer_identity: Arc<PeerIdentity>,
    remote_addr: SocketAddr,
    router: Arc<Router>,
    task_tracker: TaskTracker,
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

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    pub async fn serve(self, cancellation_token: CancellationToken) {
        let builder = Arc::new(Builder::new(TokioExecutor::new()));
        let graceful = GracefulShutdown::new();
        let task_tracker = TaskTracker::new();

        while let Some(accepted) = cancellation_token
            .run_until_cancelled(self.listener.accept())
            .await
        {
            accept_outcome(accepted).map(|connection| {
                self.spawn_connection(
                    &builder,
                    &graceful,
                    &cancellation_token,
                    &task_tracker,
                    connection,
                )
            });
        }

        graceful.shutdown().await;
        task_tracker.close();
        task_tracker.wait().await;
    }

    fn spawn_connection(
        &self,
        builder: &Arc<Builder<TokioExecutor>>,
        graceful: &GracefulShutdown,
        cancellation_token: &CancellationToken,
        task_tracker: &TaskTracker,
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
        let connection_token = cancellation_token.child_token();
        let task_tracker = task_tracker.clone();

        tokio::spawn(async move {
            match transport {
                BoundTransport::Plain => {
                    serve_connection(
                        builder,
                        watcher,
                        TokioIo::new(stream),
                        ConnectionContext {
                            body_limit,
                            cancellation_token: connection_token,
                            forward_targets,
                            peer_identity: Arc::new(PeerIdentity::from_peer_certificate(None)),
                            remote_addr,
                            router,
                            task_tracker,
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
                            cancellation_token: connection_token,
                            forward_targets,
                            peer_identity,
                            remote_addr,
                            router,
                            task_tracker,
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

    report_connection_outcome(
        watcher
            .watch(builder.serve_connection_with_upgrades(io, service))
            .await,
    );
}

async fn dispatch(
    ConnectionContext {
        body_limit,
        cancellation_token,
        forward_targets,
        peer_identity,
        remote_addr,
        router,
        task_tracker,
        upload_config,
    }: ConnectionContext,
    mut request: http::Request<Incoming>,
) -> http::Response<Full<Bytes>> {
    let on_upgrade = hyper::upgrade::on(&mut request);
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
        Ok(inputs) => match router
            .respond(
                Request::from_inputs(inputs).with_peer_identity(peer_identity),
                &forward_targets,
            )
            .await
        {
            ServedOutcome::Http(response) => response.into_http(),
            ServedOutcome::Upgrade(handler) => {
                let switching_response = handler.switching_response();
                let connection_token = cancellation_token.child_token();

                task_tracker.spawn(async move {
                    match on_upgrade.await {
                        Ok(upgraded) => {
                            handler.serve(TokioIo::new(upgraded), connection_token).await;
                        }
                        Err(error) => {
                            eprintln!(
                                "margaret_http: the connection upgrade did not complete: {error}"
                            );
                        }
                    }
                });

                switching_response.into_http()
            }
        },
        Err(error) => error_response(error).into_http(),
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::sync::Arc;

    use async_trait::async_trait;
    use hyper::upgrade::Upgraded;
    use hyper_util::rt::TokioIo;
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
    use crate::method_handler::MethodHandler;
    use crate::request::Request;
    use crate::request_error::RequestError;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::route_entry::RouteEntry;
    use crate::router::Router;
    use crate::served_outcome::ServedOutcome;
    use crate::server::Server;
    use crate::server_registry::ServerRegistry;
    use crate::transport_config::TransportConfig;
    use crate::upgrade_handler::UpgradeHandler;
    use crate::upload_config::UploadConfig;

    struct EchoUpgradeRoute;

    #[async_trait]
    impl Handler for EchoUpgradeRoute {
        async fn handle(&self, request: &Request) -> ResponseContinuation {
            if request
                .inputs
                .server
                .header("upgrade")
                .is_ok_and(|value| value.is_some())
            {
                ResponseContinuation::Upgrade(Box::new(EchoUpgrade))
            } else {
                ResponseContinuation::Done(Response::text(426, "Upgrade Required"))
            }
        }
    }

    struct UnconditionalUpgradeRoute;

    #[async_trait]
    impl Handler for UnconditionalUpgradeRoute {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Upgrade(Box::new(EchoUpgrade))
        }
    }

    struct EchoUpgrade;

    #[async_trait]
    impl UpgradeHandler for EchoUpgrade {
        fn switching_response(&self) -> Response {
            Response::text(101, "")
                .header("connection", "upgrade")
                .header("upgrade", "echo")
        }

        async fn serve(
            self: Box<Self>,
            mut upgraded: TokioIo<Upgraded>,
            connection_token: CancellationToken,
        ) {
            let mut buffer = [0u8; 64];

            loop {
                tokio::select! {
                    () = connection_token.cancelled() => break,
                    read = upgraded.read(&mut buffer) => match read {
                        Ok(0) | Err(_) => break,
                        Ok(count) => {
                            if upgraded.write_all(&buffer[..count]).await.is_err() {
                                break;
                            }
                        }
                    },
                }
            }
        }
    }

    fn registry_with_echo() -> Arc<ServerRegistry> {
        Arc::new(ServerRegistry::new(vec![Server::new(
            "test",
            "127.0.0.1:0".to_string(),
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(vec![
                RouteEntry::new(
                    "/echo",
                    vec![MethodHandler::new("GET", Arc::new(EchoUpgradeRoute))],
                ),
                RouteEntry::new(
                    "/upgrade-always",
                    vec![MethodHandler::new("GET", Arc::new(UnconditionalUpgradeRoute))],
                ),
            ])
            .expect("the echo routes register cleanly"),
        )]))
    }

    async fn open(address: SocketAddr, request: &str) -> TcpStream {
        let mut stream = TcpStream::connect(address)
            .await
            .expect("the client connects to the bound server");

        stream
            .write_all(request.as_bytes())
            .await
            .expect("the request reaches the server");

        stream
    }

    async fn read_http_head(stream: &mut TcpStream) -> String {
        let mut head = Vec::new();
        let mut byte = [0u8; 1];

        while !head.ends_with(b"\r\n\r\n") {
            let read = stream
                .read(&mut byte)
                .await
                .expect("the response head is readable");

            assert_ne!(read, 0, "the server closed before finishing the response head");

            head.push(byte[0]);
        }

        String::from_utf8_lossy(&head).into_owned()
    }

    async fn upgrade_and_echo(address: SocketAddr, path: &str, payload: &[u8]) -> (String, Vec<u8>) {
        let mut stream = open(
            address,
            &format!(
                "GET {path} HTTP/1.1\r\nHost: test\r\nConnection: Upgrade\r\nUpgrade: echo\r\n\r\n"
            ),
        )
        .await;
        let head = read_http_head(&mut stream).await;

        stream
            .write_all(payload)
            .await
            .expect("the payload reaches the upgraded connection");

        let mut echoed = vec![0u8; payload.len()];

        stream
            .read_exact(&mut echoed)
            .await
            .expect("the payload is echoed back");

        (head, echoed)
    }

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
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(vec![RouteEntry::new(
                "/",
                vec![MethodHandler::new("GET", Arc::new(PlainOk))],
            )])
            .expect("the route entries register cleanly"),
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
            TransportConfig::Plain,
            UploadConfig::Disabled,
            BodyLimit::default(),
            Router::build(Vec::new()).expect("an empty router builds"),
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
        let address = bound
            .local_addr()
            .expect("the bound listener reports its address");
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

    #[tokio::test]
    async fn upgrades_a_connection_and_echoes_frames_over_it() {
        let bound = BoundServer::bind(registry_with_echo(), empty_forward_targets(), Arc::from("test"))
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound
            .local_addr()
            .expect("the bound listener reports its address");
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let (headers, echoed) = upgrade_and_echo(address, "/echo", b"storyboard").await;

        assert!(headers.contains(" 101 "));
        assert_eq!(echoed, b"storyboard");

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }

    #[tokio::test]
    async fn responds_with_upgrade_required_when_the_handshake_headers_are_absent() {
        let bound = BoundServer::bind(registry_with_echo(), empty_forward_targets(), Arc::from("test"))
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound
            .local_addr()
            .expect("the bound listener reports its address");
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let response = exchange(
            address,
            b"GET /echo HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
            false,
        )
        .await;

        assert!(response.contains(" 426 "));

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }

    #[tokio::test]
    async fn logs_and_survives_when_the_upgrade_cannot_complete() {
        let bound = BoundServer::bind(registry_with_echo(), empty_forward_targets(), Arc::from("test"))
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound
            .local_addr()
            .expect("the bound listener reports its address");
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let mut stream = open(address, "GET /upgrade-always HTTP/1.1\r\nHost: test\r\n\r\n").await;
        let head = read_http_head(&mut stream).await;

        assert!(head.contains(" 101 "));

        drop(stream);

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }

    #[test]
    fn expect_http_yields_the_switching_response_for_an_upgrade_outcome() {
        let outcome = ServedOutcome::Upgrade(Box::new(EchoUpgrade));

        assert_eq!(outcome.expect_http().status(), 101);
    }
}
