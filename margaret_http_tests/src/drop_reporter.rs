use tokio::sync::mpsc::UnboundedSender;

use crate::responder_outcome::ResponderOutcome;

pub struct DropReporter {
    pub outcomes: UnboundedSender<ResponderOutcome>,
}

impl Drop for DropReporter {
    fn drop(&mut self) {
        self.outcomes
            .send(ResponderOutcome::Dropped)
            .expect("the test observes the responder future being released");
    }
}
