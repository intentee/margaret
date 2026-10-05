use async_trait::async_trait;

use margaret_http::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

pub struct FailingHandler;

#[async_trait]
impl HeadHandler for FailingHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Err(HandlerError::consumer(anyhow::anyhow!(
            "secret database endpoint"
        )))
    }
}
