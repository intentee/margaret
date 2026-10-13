use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use margaret_handler_error::handler_error::HandlerError;

use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;
use crate::web_socket_driver_sender::WebSocketDriverSender;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub(crate) struct WebSocketUpgradeTerminal {
    cancellation_token: CancellationToken,
    on_upgrade: OnUpgrade,
    upgrade: Arc<dyn WebSocketUpgrade>,
    driver_sender: WebSocketDriverSender,
}

impl WebSocketUpgradeTerminal {
    pub(crate) fn new(
        upgrade: Arc<dyn WebSocketUpgrade>,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
        driver_sender: WebSocketDriverSender,
    ) -> Self {
        Self {
            cancellation_token,
            on_upgrade,
            upgrade,
            driver_sender,
        }
    }
}

#[async_trait]
impl OneShotHandler for WebSocketUpgradeTerminal {
    async fn handle(
        self: Box<Self>,
        request: &Request,
    ) -> Result<ResponseContinuation, HandlerError> {
        let Self {
            cancellation_token,
            driver_sender,
            on_upgrade,
            upgrade,
        } = *self;

        Ok(upgrade
            .upgrade(request, on_upgrade, cancellation_token, driver_sender)
            .await)
    }
}
