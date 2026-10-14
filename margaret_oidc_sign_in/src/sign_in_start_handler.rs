use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::handles_head::HandlesHead;
use margaret_http::reported_unavailability::reported_unavailability;
use margaret_http::request::Request;
use margaret_http::response::Response;

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
impl HandlesHead for SignInStartHandler {
    async fn handle(&self, _request: &Request) -> Result<Response, HandlerError> {
        Ok(match self.flow.begin().await {
            SignInBeginning::Redirected(response) => response,
            SignInBeginning::Unavailable(unavailability) => {
                reported_unavailability(&unavailability)
            }
        })
    }
}
