use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::redirect::Redirect;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_server_origin::route_url::RouteUrl;

use crate::issued_sessions::IssuedSessions;

pub struct SessionSignOutEndpoint {
    landing: String,
    sessions: Arc<IssuedSessions>,
}

impl SessionSignOutEndpoint {
    #[must_use]
    pub fn create(sessions: Arc<IssuedSessions>, landing: RouteUrl) -> Self {
        Self {
            landing: String::from(landing),
            sessions,
        }
    }
}

#[async_trait]
impl HeadHandler for SessionSignOutEndpoint {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.sessions
            .sign_out(request)
            .await
            .map(|removal| {
                removal.precede(ResponseContinuation::from(Redirect::see_other(
                    self.landing.clone(),
                )))
            })
            .map_err(|error| HandlerError::consumer(anyhow::Error::from(error)))
    }
}
