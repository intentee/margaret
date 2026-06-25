use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

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

    pub async fn serve(self) -> ! {
        loop {
            let (stream, _remote) = self
                .listener
                .accept()
                .await
                .expect("the listener accepts connections");

            tokio::spawn(serve_connection(stream, self.app.clone()));
        }
    }
}

async fn serve_connection(
    stream: TcpStream,
    app: Arc<dyn Handler>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = TowerToHyperService::new(tower::service_fn(
        move |request: http::Request<Incoming>| {
            let app = app.clone();

            async move { Ok::<_, Infallible>(dispatch(app, request).await) }
        },
    ));

    Builder::new(TokioExecutor::new())
        .serve_connection(TokioIo::new(stream), service)
        .await
}

async fn dispatch(
    app: Arc<dyn Handler>,
    request: http::Request<Incoming>,
) -> http::Response<Full<Bytes>> {
    let Some(method) = Method::from_http(request.method()) else {
        return Response::text(405, "Method Not Allowed").into_http();
    };

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
    let mut handled = Request::new(method, request.uri().path().to_string());

    handled.set_headers(headers);

    app.handle(handled).await.into_http()
}
