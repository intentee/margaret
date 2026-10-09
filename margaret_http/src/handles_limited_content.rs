use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::body_limit::BodyLimit;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait HandlesLimitedContent: Send + Sync {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
        limit: BodyLimit,
    ) -> Result<ResponseContinuation, HandlerError>;
}
