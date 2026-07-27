use std::sync::Arc;

use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use async_trait::async_trait;

use crate::handler_error::HandlerError;
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use http::Method;
    use hyper::upgrade::OnUpgrade;
    use tokio_util::sync::CancellationToken;

    use super::WebSocketUpgradeTerminal;
    use crate::forward_targets::ForwardTargets;
    use crate::one_shot_handler::OneShotHandler;
    use crate::request::Request;
    use crate::resolve_continuation::resolve_continuation;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::web_socket_driver_channel::web_socket_driver_channel;
    use crate::web_socket_driver_sender::WebSocketDriverSender;
    use crate::web_socket_upgrade::WebSocketUpgrade;

    struct AcceptingUpgrade;

    #[async_trait]
    impl WebSocketUpgrade for AcceptingUpgrade {
        async fn upgrade(
            self: Arc<Self>,
            _handshake: &Request,
            _on_upgrade: OnUpgrade,
            _cancellation_token: CancellationToken,
            _driver_sender: WebSocketDriverSender,
        ) -> ResponseContinuation {
            ResponseContinuation::from(Response::text(101, ""))
        }
    }

    async fn status_of(outcome: ResponseContinuation) -> u16 {
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));

        resolve_continuation(
            &forward_targets,
            Request::new(Method::GET, "/chat".to_string()),
            outcome,
        )
        .await
        .expect("the continuation resolves")
        .into_http()
        .status()
        .as_u16()
    }

    #[tokio::test]
    async fn consumes_the_terminal_when_it_upgrades() {
        let mut request = hyper::Request::new(());
        let on_upgrade = hyper::upgrade::on(&mut request);
        let terminal = Box::new(WebSocketUpgradeTerminal::new(
            Arc::new(AcceptingUpgrade),
            on_upgrade,
            CancellationToken::new(),
            web_socket_driver_channel().0,
        ));
        let handshake = Request::new(Method::GET, "/chat".to_string());

        assert_eq!(
            status_of(
                terminal
                    .handle(&handshake)
                    .await
                    .expect("the upgrade is accepted")
            )
            .await,
            101
        );
    }
}
