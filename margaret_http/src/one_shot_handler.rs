use async_trait::async_trait;

use crate::handler_error::HandlerError;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub(crate) trait OneShotHandler: Send {
    async fn handle(
        self: Box<Self>,
        request: &Request,
    ) -> Result<ResponseContinuation, HandlerError>;
}
