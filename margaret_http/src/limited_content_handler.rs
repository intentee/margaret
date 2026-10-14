use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::body_limit::BodyLimit;
use crate::content_handler::ContentHandler;
use crate::handles_limited_content::HandlesLimitedContent;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

struct LimitedContent<THandler> {
    handler: Arc<THandler>,
    limit: BodyLimit,
}

#[async_trait]
impl<TTarget, THandler> ContentHandler<TTarget> for LimitedContent<THandler>
where
    THandler: HandlesLimitedContent,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        self.handler
            .handle(request, body, self.limit)
            .await
            .map(ResponseContinuation::Done)
    }
}

pub fn limited_content_handler<TTarget, THandler: HandlesLimitedContent + 'static>(
    handler: Arc<THandler>,
    limit: BodyLimit,
) -> Arc<dyn ContentHandler<TTarget>> {
    Arc::new(LimitedContent { handler, limit })
}
