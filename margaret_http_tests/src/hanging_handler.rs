use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

#[derive(Default)]
pub struct HangingHandler {
    pub release: CancellationToken,
    pub request_received: CancellationToken,
}

#[async_trait]
impl HeadHandler for HangingHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.request_received.cancel();
        self.release.cancelled().await;

        Ok(ResponseContinuation::Done(Response::text(200, "released")))
    }
}
