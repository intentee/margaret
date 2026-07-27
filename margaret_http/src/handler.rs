use async_trait::async_trait;

use crate::handler_error::HandlerError;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError>;
}
