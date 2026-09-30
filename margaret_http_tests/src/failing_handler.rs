use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response_continuation::ResponseContinuation;

pub struct FailingHandler;

#[async_trait]
impl Handler for FailingHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        Err(HandlerError::consumer(anyhow::anyhow!(
            "secret database endpoint"
        )))
    }
}
