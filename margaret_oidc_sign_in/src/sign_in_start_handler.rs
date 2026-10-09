use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::sign_in_beginning::SignInBeginning;
use crate::sign_in_flow::SignInFlow;

pub struct SignInStartHandler {
    flow: Arc<SignInFlow>,
}

impl SignInStartHandler {
    #[must_use]
    pub fn create(flow: Arc<SignInFlow>) -> Self {
        Self { flow }
    }
}

#[async_trait]
impl HeadHandler for SignInStartHandler {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::from(match self.flow.begin().await {
            SignInBeginning::Redirected(response) => response,
            SignInBeginning::Unavailable(unavailability) => {
                Response::text(503, unavailability.to_string())
            }
        }))
    }
}
