use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait HeadHandler<TTarget>: Send + Sync {
    async fn handle(
        &self,
        request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError>;
}
