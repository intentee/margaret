use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct LimitReportingHandler;

#[async_trait]
impl HandlesLimitedContent for LimitReportingHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
        limit: BodyLimit,
    ) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(Response::text(
            200,
            limit.max_bytes().to_string(),
        )))
    }
}
