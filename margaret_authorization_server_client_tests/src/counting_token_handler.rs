use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use serde_json::json;

use margaret_http::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub struct CountingTokenHandler {
    pub expires_in: Option<u64>,
    pub issued: AtomicUsize,
}

#[async_trait]
impl HeadHandler for CountingTokenHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
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
