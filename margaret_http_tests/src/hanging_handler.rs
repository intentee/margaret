use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

#[derive(Default)]
pub struct HangingHandler {
    pub release: CancellationToken,
    pub request_received: CancellationToken,
}

#[async_trait]
impl Handler for HangingHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        self.request_received.cancel();
        self.release.cancelled().await;

        Ok(ResponseContinuation::Done(Response::text(200, "released")))
    }
}
