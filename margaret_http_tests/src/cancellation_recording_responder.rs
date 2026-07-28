use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedSender;

use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::responder_outcome::ResponderOutcome;

struct DropReporter {
    outcomes: UnboundedSender<ResponderOutcome>,
}

impl Drop for DropReporter {
    fn drop(&mut self) {
        self.outcomes
            .send(ResponderOutcome::Dropped)
            .expect("the test observes the responder future being released");
    }
}

pub struct CancellationRecordingResponder {
    pub outcomes: UnboundedSender<ResponderOutcome>,
    pub started: UnboundedSender<()>,
}

#[async_trait]
impl Handler for CancellationRecordingResponder {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let _drop_reporter = DropReporter {
            outcomes: self.outcomes.clone(),
        };

        self.started
            .send(())
            .expect("the test observes the responder start");

        request.cancellation_token().cancelled().await;

        self.outcomes
            .send(ResponderOutcome::CleanedUp)
            .expect("the test observes the responder cleanup");

        Ok(ResponseContinuation::Done(Response::text(
            200,
            "cleaned up",
        )))
    }
}
