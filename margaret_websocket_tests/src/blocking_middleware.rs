use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct BlockingMiddleware;

#[async_trait]
impl HttpMiddleware for BlockingMiddleware {
    async fn process(
        &self,
        _request: &Request,
        _next: Next,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(Response::forbidden()))
    }
}
