use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub(crate) struct WebSocketUpgradeTerminal {
    cancellation_token: CancellationToken,
    on_upgrade: Mutex<Option<OnUpgrade>>,
    upgrade: Arc<dyn WebSocketUpgrade>,
}

impl WebSocketUpgradeTerminal {
    pub(crate) fn new(
        upgrade: Arc<dyn WebSocketUpgrade>,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            cancellation_token,
            on_upgrade: Mutex::new(Some(on_upgrade)),
            upgrade,
        }
    }
}

#[async_trait]
impl Handler for WebSocketUpgradeTerminal {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let on_upgrade = self.on_upgrade.lock().await.take();

        match on_upgrade {
            Some(on_upgrade) => Ok(self
                .upgrade
                .clone()
                .upgrade(request, on_upgrade, self.cancellation_token.clone())
                .await),
            None => Err(HandlerError::RepeatedWebSocketUpgrade),
        }
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
    use crate::handler::Handler;
    use crate::request::Request;
    use crate::resolve_continuation::resolve_continuation;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::web_socket_upgrade::WebSocketUpgrade;

    struct AcceptingUpgrade;

    #[async_trait]
    impl WebSocketUpgrade for AcceptingUpgrade {
        async fn upgrade(
            self: Arc<Self>,
            _handshake: &Request,
            _on_upgrade: OnUpgrade,
            _cancellation_token: CancellationToken,
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
    async fn upgrades_once_then_rejects_a_second_entry() {
        let mut request = hyper::Request::new(());
        let on_upgrade = hyper::upgrade::on(&mut request);
        let terminal = WebSocketUpgradeTerminal::new(
            Arc::new(AcceptingUpgrade),
            on_upgrade,
            CancellationToken::new(),
        );
        let handshake = Request::new(Method::GET, "/chat".to_string());

        assert_eq!(
            status_of(
                terminal
                    .handle(&handshake)
                    .await
                    .expect("the first upgrade is accepted")
            )
            .await,
            101
        );
        assert!(terminal.handle(&handshake).await.is_err());
    }
}
