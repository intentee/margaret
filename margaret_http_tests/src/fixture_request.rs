use std::net::SocketAddr;
use std::sync::Arc;

use cookie::Cookie;
use http::HeaderMap;
use http::HeaderValue;
use http::Method;
use http::header::COOKIE;

use margaret_http::request::Request;
use margaret_http::request_outcome::RequestOutcome;
use margaret_http::request_rejection::RequestRejection;
use margaret_http::router::Router;
use margaret_http::server::Server;
use margaret_http::transport_config::TransportConfig;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_peer_identity::peer_identity::PeerIdentity;

pub struct FixtureRequest {
    pub headers: HeaderMap,
    pub method: Method,
    pub peer_identity: PeerIdentity,
    pub target: String,
    pub upload_config: UploadConfig,
}

impl FixtureRequest {
    #[must_use]
    pub fn new(method: Method, target: &str) -> Self {
        Self {
            headers: HeaderMap::new(),
            method,
            peer_identity: PeerIdentity::Anonymous,
            target: target.to_string(),
            upload_config: UploadConfig::Disabled,
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture request head is ambiguous.
    #[must_use]
    pub fn into_request(self) -> Request {
        self.into_result()
            .expect("the fixture request head is unambiguous")
    }

    /// # Errors
    ///
    /// Returns the `RequestRejection` of an ambiguous fixture request head.
    ///
    /// # Panics
    ///
    /// Panics when the fixture request head cannot be built.
    pub fn into_result(self) -> Result<Request, RequestRejection> {
        let mut builder = http::Request::builder()
            .method(self.method)
            .uri(self.target)
            .header(http::header::HOST, "fixture.test");

        for (name, value) in &self.headers {
            builder = builder.header(name, value);
        }

        let parts = builder
            .body(())
            .expect("the fixture request head builds")
            .into_parts()
            .0;
        let server = Arc::new(Server::new(
            "127.0.0.1:0".to_string(),
            TransportConfig::Plain,
            self.upload_config,
            Router::build(Vec::new()).expect("an empty router builds"),
        ));

        match Request::from_head(
            server,
            parts,
            SocketAddr::from(([192, 0, 2, 1], 4000)),
            Arc::new(self.peer_identity),
        ) {
            RequestOutcome::Parsed(request) => Ok(request),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    /// # Panics
    ///
    /// Panics when the cookies cannot be written as a cookie header.
    #[must_use]
    pub fn presenting_cookies(mut self, cookies: &[Cookie<'_>]) -> Self {
        if !cookies.is_empty() {
            self.headers.insert(
                COOKIE,
                HeaderValue::from_str(
                    &cookies
                        .iter()
                        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
                        .collect::<Vec<String>>()
                        .join("; "),
                )
                .expect("the cookies form a header value"),
            );
        }

        self
    }
}
