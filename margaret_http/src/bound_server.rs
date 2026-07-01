use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use hyper_util::server::graceful::GracefulShutdown;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use crate::handler::Handler;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;

pub struct BoundServer {
    app: Arc<dyn Handler>,
    listener: TcpListener,
}

impl BoundServer {
    pub(crate) fn new(app: Arc<dyn Handler>, listener: TcpListener) -> Self {
        Self { app, listener }
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
            let (stream, _remote) = accepted.expect("the listener accepts connections");
            let app = self.app.clone();
            let builder = builder.clone();
            let watcher = graceful.watcher();

            tokio::spawn(async move {
                let service = TowerToHyperService::new(tower::service_fn(
                    move |request: http::Request<Incoming>| {
                        let app = app.clone();

                        async move { Ok::<_, Infallible>(dispatch(app, request).await) }
                    },
                ));
                let connection = builder.serve_connection(TokioIo::new(stream), service);

                if let Err(error) = watcher.watch(connection).await {
                    eprintln!("margaret_http: connection error: {error}");
                }
            });
        }

        graceful.shutdown().await;
    }
}

async fn dispatch(
    app: Arc<dyn Handler>,
    request: http::Request<Incoming>,
) -> http::Response<Full<Bytes>> {
    let Some(method) = Method::from_http(request.method()) else {
        return Response::text(405, "Method Not Allowed").into_http();
    };

    let path = request.uri().path().to_string();
    let (parts, incoming) = request.into_parts();
    let body = match incoming.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(source) => return Response::text(400, format!("Bad Request: {source}")).into_http(),
    };
    let mut handled = Request::new(method, path);

    handled.set_headers(parts.headers);
    handled.set_body(body);

    app.handle(handled).await.into_http()
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use tokio::io::AsyncReadExt;
    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;
    use tokio_util::sync::CancellationToken;

    use crate::router::Router;
    use crate::server::Server;

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

    #[tokio::test(flavor = "multi_thread")]
    async fn serves_connections_until_cancellation() {
        let bound = Server::new(Router::empty())
            .bind("127.0.0.1:0")
            .await
            .expect("the server binds to an ephemeral port");
        let address = bound.local_addr();
        let cancellation_token = CancellationToken::new();
        let serving = tokio::spawn(bound.serve(cancellation_token.clone()));

        let unmatched = exchange(
            address,
            b"GET /missing HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
            false,
        )
        .await;
        assert!(unmatched.contains(" 404 "));

        let unsupported = exchange(
            address,
            b"OPTIONS / HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
            false,
        )
        .await;
        assert!(unsupported.contains(" 405 "));

        let truncated = exchange(
            address,
            b"POST / HTTP/1.1\r\nHost: test\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort",
            true,
        )
        .await;
        assert!(truncated.contains(" 400 "));

        let malformed = exchange(address, b"this is not a valid request line\r\n\r\n", true).await;
        assert!(!malformed.contains(" 200 "));

        let recovered = exchange(
            address,
            b"GET /missing HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
            false,
        )
        .await;
        assert!(recovered.contains(" 404 "));

        cancellation_token.cancel();

        serving.await.expect("the server task finishes cleanly");
    }
}
