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

                let _ = watcher.watch(connection).await;
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
    let headers = request
        .headers()
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (name.as_str().to_string(), value.to_string()))
        })
        .collect();
    let body = match request.into_body().collect().await {
        Ok(collected) => String::from_utf8_lossy(&collected.to_bytes()).into_owned(),
        Err(source) => return Response::text(400, format!("Bad Request: {source}")).into_http(),
    };
    let mut handled = Request::new(method, path);

    handled.set_headers(headers);
    handled.set_body(body);

    app.handle(handled).await.into_http()
}
