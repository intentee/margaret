use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::ci::runner_report::RunnerReport;
use crate::ci::runner_session::RunnerSession;
use crate::ci::who_am_i::WhoAmI;

#[singleton]
pub struct WhoAmIResponder;

impl WhoAmIResponder {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for WhoAmIResponder {
    type Message = WhoAmI;
    type Session = RunnerSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<RunnerSession>,
        message: RequestEnvelope<WhoAmI>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.response(RunnerReport {
                repository: session.repository().to_string(),
            }))
            .await?;

        Ok(())
    }
}
