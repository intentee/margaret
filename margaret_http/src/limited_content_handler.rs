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
impl<THandler: HandlesLimitedContent> ContentHandler for LimitedContent<THandler> {
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        self.handler.handle(request, body, self.limit).await
    }
}

pub fn limited_content_handler<THandler: HandlesLimitedContent + 'static>(
    handler: Arc<THandler>,
    limit: BodyLimit,
) -> Arc<dyn ContentHandler> {
    Arc::new(LimitedContent { handler, limit })
}
