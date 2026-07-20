use std::sync::Arc;

use tokio::sync::Mutex;

use margaret_macros::build_for_session;
use margaret_macros::websocket_session;

use crate::greeter::Greeter;

#[websocket_session(path = "/storyboard/{topic}", server = "public")]
pub struct StoryboardSession {
    greeter: Arc<dyn Greeter>,
    topic: String,
    turns: Mutex<Vec<String>>,
}

impl StoryboardSession {
    #[build_for_session]
    #[must_use]
    pub fn build_for_session(
        greeter: Arc<dyn Greeter>,
        #[route_parameter(from = "topic")] topic: String,
    ) -> Self {
        Self {
            greeter,
            topic,
            turns: Mutex::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn greeting(&self) -> String {
        self.greeter.greet()
    }

    pub async fn record(&self, turn: String) {
        self.turns.lock().await.push(turn);
    }

    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub async fn turn_count(&self) -> usize {
        self.turns.lock().await.len()
    }
}
