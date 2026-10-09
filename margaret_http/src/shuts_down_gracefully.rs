use std::error::Error;
use std::pin::Pin;

use http_body::Body;
use hyper::body::Incoming;
use hyper::rt::Read;
use hyper::rt::Write;
use hyper::server::conn::http1::UpgradeableConnection;
use hyper::service::HttpService;

pub(crate) trait ShutsDownGracefully {
    fn shut_down_gracefully(self: Pin<&mut Self>);
}

impl<TIo, TService, TBody> ShutsDownGracefully for UpgradeableConnection<TIo, TService>
where
    TService: HttpService<Incoming, ResBody = TBody>,
    TService::Error: Into<Box<dyn Error + Send + Sync>>,
    TIo: Read + Write + Unpin,
    TBody: Body + 'static,
    TBody::Error: Into<Box<dyn Error + Send + Sync>>,
{
    fn shut_down_gracefully(self: Pin<&mut Self>) {
        self.graceful_shutdown();
    }
}
