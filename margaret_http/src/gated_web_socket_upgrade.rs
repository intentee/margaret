use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use crate::forward_targets::ForwardTargets;
use crate::http_middleware::HttpMiddleware;
use crate::request::Request;
use crate::response::Response;
use crate::run_upgrade_gate::run_upgrade_gate;
use crate::upgrade_gate_outcome::UpgradeGateOutcome;
use crate::web_socket_upgrade::WebSocketUpgrade;

pub struct GatedWebSocketUpgrade {
    inner: Arc<dyn WebSocketUpgrade>,
    middleware: Vec<Arc<dyn HttpMiddleware>>,
}

impl GatedWebSocketUpgrade {
    #[must_use]
    pub fn new(inner: Arc<dyn WebSocketUpgrade>, middleware: Vec<Arc<dyn HttpMiddleware>>) -> Self {
        Self { inner, middleware }
    }
}

#[async_trait]
impl WebSocketUpgrade for GatedWebSocketUpgrade {
    async fn upgrade(
        self: Arc<Self>,
        handshake: Request,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
        forward_targets: &Arc<ForwardTargets>,
    ) -> Response {
        match run_upgrade_gate(&self.middleware, forward_targets, handshake).await {
            UpgradeGateOutcome::ShortCircuit(response) => response,
            UpgradeGateOutcome::Proceed(handshake) => {
                self.inner
                    .clone()
                    .upgrade(*handshake, on_upgrade, cancellation_token, forward_targets)
                    .await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use async_trait::async_trait;
    use http::Method;
    use hyper::upgrade::OnUpgrade;
    use tokio_util::sync::CancellationToken;

    use super::GatedWebSocketUpgrade;
    use crate::forward::Forward;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::http_middleware::HttpMiddleware;
    use crate::named_handler::NamedHandler;
    use crate::next::Next;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::web_socket_upgrade::WebSocketUpgrade;

    struct PassThrough;

    #[async_trait]
    impl HttpMiddleware for PassThrough {
        async fn process(&self, request: &Request, next: Next) -> ResponseContinuation {
            next.run(request).await
        }
    }

    struct RespondsWith {
        status: u16,
    }

    #[async_trait]
    impl HttpMiddleware for RespondsWith {
        async fn process(&self, _request: &Request, _next: Next) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(self.status, "short circuit"))
        }
    }

    struct ForwardsTo {
        name: &'static str,
    }

    #[async_trait]
    impl HttpMiddleware for ForwardsTo {
        async fn process(&self, _request: &Request, _next: Next) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new(self.name, HashMap::new()))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(222, "target"))
        }
    }

    struct RecordsWhetherItRan {
        ran: Arc<AtomicBool>,
    }

    #[async_trait]
    impl WebSocketUpgrade for RecordsWhetherItRan {
        async fn upgrade(
            self: Arc<Self>,
            _handshake: Request,
            _on_upgrade: OnUpgrade,
            _cancellation_token: CancellationToken,
            _forward_targets: &Arc<ForwardTargets>,
        ) -> Response {
            self.ran.store(true, Ordering::SeqCst);

            Response::text(101, "upgraded")
        }
    }

    async fn gate_status(
        inner: Arc<dyn WebSocketUpgrade>,
        middleware: Vec<Arc<dyn HttpMiddleware>>,
        forward_targets: Arc<ForwardTargets>,
    ) -> u16 {
        let mut request = hyper::Request::new(());
        let on_upgrade = hyper::upgrade::on(&mut request);

        Arc::new(GatedWebSocketUpgrade::new(inner, middleware))
            .upgrade(
                Request::new(Method::GET, "/chat".to_string()),
                on_upgrade,
                CancellationToken::new(),
                &forward_targets,
            )
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn delegates_to_the_inner_upgrade_when_the_middleware_proceeds() {
        let ran = Arc::new(AtomicBool::new(false));
        let status = gate_status(
            Arc::new(RecordsWhetherItRan { ran: ran.clone() }),
            vec![Arc::new(PassThrough)],
            Arc::new(ForwardTargets::new(Vec::new())),
        )
        .await;

        assert_eq!(status, 101);
        assert!(ran.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn returns_the_short_circuit_without_running_the_inner_upgrade() {
        let ran = Arc::new(AtomicBool::new(false));
        let status = gate_status(
            Arc::new(RecordsWhetherItRan { ran: ran.clone() }),
            vec![Arc::new(RespondsWith { status: 403 })],
            Arc::new(ForwardTargets::new(Vec::new())),
        )
        .await;

        assert_eq!(status, 403);
        assert!(!ran.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn resolves_a_forward_without_running_the_inner_upgrade() {
        let ran = Arc::new(AtomicBool::new(false));
        let status = gate_status(
            Arc::new(RecordsWhetherItRan { ran: ran.clone() }),
            vec![Arc::new(ForwardsTo { name: "target" })],
            Arc::new(ForwardTargets::new(vec![NamedHandler::new(
                "target",
                Arc::new(Target),
            )])),
        )
        .await;

        assert_eq!(status, 222);
        assert!(!ran.load(Ordering::SeqCst));
    }
}
