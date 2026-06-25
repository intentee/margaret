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
use crate::router::Router;

pub struct Server {
    app: Arc<dyn Handler>,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Self {
            app: Arc::new(router),
        }
    }

    pub async fn bind(self, address: &str) -> std::io::Result<BoundServer> {
        let listener = TcpListener::bind(address).await?;

        Ok(BoundServer {
            app: self.app,
            listener,
        })
    }
}

pub struct BoundServer {
    app: Arc<dyn Handler>,
    listener: TcpListener,
}

impl BoundServer {
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

#[cfg(test)]
mod tests {
    use super::Server;
    use crate::router::Router;

    #[tokio::test]
    async fn bind_fails_for_an_invalid_address() {
        let server = Server::new(Router::empty());

        assert!(server.bind("this is not an address").await.is_err());
    }
}
