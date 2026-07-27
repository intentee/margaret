use async_trait::async_trait;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::request::Request;
use margaret_http::route_origin::RouteOrigin;
use margaret_http::url_segment::UrlSegment;
use margaret_websocket::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct RedirectingSessionFactory {
    origin: RouteOrigin,
}

impl RedirectingSessionFactory {
    #[must_use]
    pub fn new(origin: RouteOrigin) -> Self {
        Self { origin }
    }
}

#[async_trait]
impl WebSocketSessionFactory for RedirectingSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(WebSocketSessionCreationOutcome::Interrupted(
            ForwardableRoute::new(self.origin.clone(), vec![UrlSegment::Literal("/sign-in")])
                .see_other()
                .into(),
        ))
    }
}
