use serde::Serialize;

use margaret::framework::macros::websocket_message;

#[websocket_message(response, method = "runner_report")]
#[derive(Serialize)]
pub struct RunnerReport {
    pub repository: String,
}
