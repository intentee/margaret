use std::sync::OnceLock;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::static_handler::StaticHandler;

pub struct TokenEndpoint {
    pub answer: OnceLock<StaticHandler>,
}

#[async_trait]
impl HeadHandler for TokenEndpoint {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.answer.wait().handle(request).await
    }
}
