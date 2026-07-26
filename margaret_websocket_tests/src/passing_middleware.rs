use async_trait::async_trait;

use margaret_http::handler_error::HandlerError;
use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

pub struct PassingMiddleware;

#[async_trait]
impl HttpMiddleware for PassingMiddleware {
    async fn process(
        &self,
        request: &Request,
        next: Next,
    ) -> Result<ResponseContinuation, HandlerError> {
        next.run(request).await
    }
}
