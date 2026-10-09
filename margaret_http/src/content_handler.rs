use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait ContentHandler: Send + Sync {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError>;
}
