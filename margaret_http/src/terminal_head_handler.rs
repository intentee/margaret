use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::handles_head::HandlesHead;
use crate::head_handler::HeadHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct TerminalHead<THandler: ?Sized> {
    handler: Arc<THandler>,
}

#[async_trait]
impl<TTarget, THandler> HeadHandler<TTarget> for TerminalHead<THandler>
where
    THandler: HandlesHead + ?Sized,
{
    async fn handle(
        &self,
        request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        self.handler
            .handle(request)
            .await
            .map(ResponseContinuation::Done)
    }
}

pub fn terminal_head_handler<TTarget, THandler: HandlesHead + ?Sized + 'static>(
    handler: Arc<THandler>,
) -> Arc<dyn HeadHandler<TTarget>> {
    Arc::new(TerminalHead { handler })
}
