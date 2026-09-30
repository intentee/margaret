use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use serde_json::json;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct CountingTokenHandler {
    pub expires_in: Option<u64>,
    pub issued: AtomicUsize,
}

#[async_trait]
impl Handler for CountingTokenHandler {
    async fn handle(
        &self,
        _request: &Request,
        _body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        let issued = self.issued.fetch_add(1, Ordering::SeqCst) + 1;

        Ok(ResponseContinuation::Done(Response::json(
            200,
            &json!({
                "access_token": format!("token-{issued}"),
                "expires_in": self.expires_in,
                "token_type": "Bearer",
            }),
        )))
    }
}
