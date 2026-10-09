use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct RedirectingHandler {
    pub location: &'static str,
}

#[async_trait]
impl HeadHandler for RedirectingHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            Response::text(302, "redirected").header("location", self.location),
        ))
    }
}
