use async_trait::async_trait;
use std::future::pending;
use tokio::sync::mpsc::UnboundedSender;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

use crate::drop_reporter::DropReporter;
use crate::responder_outcome::ResponderOutcome;

pub struct UncooperativeResponder {
    pub outcomes: UnboundedSender<ResponderOutcome>,
    pub started: UnboundedSender<()>,
}

#[async_trait]
impl Handler for UncooperativeResponder {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let _drop_reporter = DropReporter {
            outcomes: self.outcomes.clone(),
        };

        self.started
            .send(())
            .expect("the test observes the responder start");

        pending().await
    }
}
