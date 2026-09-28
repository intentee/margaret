use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct StaticHandler {
    pub body: Vec<u8>,
    pub content_type: &'static str,
    pub status: u16,
}

#[async_trait]
impl Handler for StaticHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(Response::bytes(
            self.status,
            self.content_type,
            self.body.clone(),
        )))
    }
}
