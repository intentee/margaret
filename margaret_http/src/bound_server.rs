use std::convert::Infallible;
use std::fmt::Display;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http::request::Parts;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use hyper_util::server::graceful::GracefulShutdown;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::method::Method;
use crate::request::Request;
use crate::request_error::RequestError;
use crate::request_inputs::RequestInputs;
use crate::response::Response;
use crate::router::Router;
use crate::upload_config::UploadConfig;

pub struct BoundServer {
    app: Arc<Router>,
    listener: TcpListener,
    upload_config: Arc<UploadConfig>,
}

impl BoundServer {
    pub(crate) fn new(
        app: Arc<Router>,
        listener: TcpListener,
        upload_config: Arc<UploadConfig>,
    ) -> Self {
        Self {
            app,
            listener,
            upload_config,
        }
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
            let (stream, remote_addr) = accepted.expect("the listener accepts connections");
            let app = self.app.clone();
            let upload_config = self.upload_config.clone();
            let builder = builder.clone();
            let watcher = graceful.watcher();

            tokio::spawn(async move {
                let service = TowerToHyperService::new(tower::service_fn(
                    move |request: http::Request<Incoming>| {
                        let app = app.clone();
                        let upload_config = upload_config.clone();

                        async move {
                            Ok::<_, Infallible>(
                                dispatch(app, upload_config, remote_addr, request).await,
                            )
                        }
                    },
                ));
                let connection = builder.serve_connection(TokioIo::new(stream), service);

                report_connection_outcome(watcher.watch(connection).await);
            });
        }

        graceful.shutdown().await;
    }
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

async fn dispatch(
    app: Arc<Router>,
    upload_config: Arc<UploadConfig>,
    remote_addr: SocketAddr,
    request: http::Request<Incoming>,
) -> http::Response<Full<Bytes>> {
    let Some(method) = Method::from_http(request.method()) else {
        return Response::text(405, "Method Not Allowed").into_http();
    };

    let (parts, incoming) = request.into_parts();
    let Parts { headers, uri, .. } = parts;

    match RequestInputs::parse(method, &uri, headers, remote_addr, incoming, &upload_config).await {
        Ok(inputs) => app.respond(Request::from_inputs(inputs)).await.into_http(),
        Err(error) => error_response(error).into_http(),
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tokio_util::sync::CancellationToken;

    use super::error_response;
    use crate::request_error::RequestError;
    use crate::router::Router;
    use crate::server::Server;
    use crate::upload_config::UploadConfig;

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
        let bound = Server::new(Router::empty())
            .bind("127.0.0.1:0", UploadConfig::Disabled)
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound.local_addr();
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let (interrupted, unmatched, unsupported, truncated) = tokio::join!(
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

        assert!(!interrupted.contains(" 200 "));
        assert!(unmatched.contains(" 404 "));
        assert!(unsupported.contains(" 405 "));
        assert!(truncated.contains(" 400 "));

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }
}
