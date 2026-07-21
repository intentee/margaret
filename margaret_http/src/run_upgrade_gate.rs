use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use async_trait::async_trait;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::layer::layer;
use crate::request::Request;
use crate::resolve_continuation::resolve_continuation;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;
use crate::upgrade_gate_outcome::UpgradeGateOutcome;

struct GateTerminal {
    reached: Arc<AtomicBool>,
}

#[async_trait]
impl Handler for GateTerminal {
    async fn handle(&self, _request: &Request) -> ResponseContinuation {
        self.reached.store(true, Ordering::SeqCst);

        ResponseContinuation::Done(Response::text(101, ""))
    }
}

pub(crate) async fn run_upgrade_gate(
    middleware: &[Arc<dyn HttpMiddleware>],
    forward_targets: &Arc<ForwardTargets>,
    handshake: Request,
) -> UpgradeGateOutcome {
    let reached = Arc::new(AtomicBool::new(false));
    let mut onion: Arc<dyn Handler> = Arc::new(GateTerminal {
        reached: reached.clone(),
    });

    for middleware_layer in middleware.iter().rev() {
        onion = layer(middleware_layer.clone(), onion);
    }

    let outcome = onion.handle(&handshake).await;

    if reached.load(Ordering::SeqCst) {
        UpgradeGateOutcome::Proceed(Box::new(handshake))
    } else {
        UpgradeGateOutcome::ShortCircuit(
            resolve_continuation(forward_targets, handshake, outcome).await,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use async_trait::async_trait;
    use http::Method;

    use super::run_upgrade_gate;
    use crate::forward::Forward;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::http_middleware::HttpMiddleware;
    use crate::named_handler::NamedHandler;
    use crate::next::Next;
    use crate::redirect::Redirect;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::upgrade_gate_outcome::UpgradeGateOutcome;

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

    struct RedirectsAway;

    #[async_trait]
    impl HttpMiddleware for RedirectsAway {
        async fn process(&self, _request: &Request, _next: Next) -> ResponseContinuation {
            ResponseContinuation::from(Redirect::see_other("http://localhost/login".to_string()))
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

    struct OverridesAfterDelegating;

    #[async_trait]
    impl HttpMiddleware for OverridesAfterDelegating {
        async fn process(&self, request: &Request, next: Next) -> ResponseContinuation {
            let _ignored = next.run(request).await;

            ResponseContinuation::Done(Response::text(500, "too late"))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(222, "target"))
        }
    }

    fn no_forward_targets() -> Arc<ForwardTargets> {
        Arc::new(ForwardTargets::new(Vec::new()))
    }

    fn handshake() -> Request {
        Request::new(Method::GET, "/chat".to_string())
    }

    struct GateSummary {
        proceeded: bool,
        response_status: Option<u16>,
        room: Option<String>,
    }

    fn summarize(outcome: UpgradeGateOutcome) -> GateSummary {
        match outcome {
            UpgradeGateOutcome::Proceed(handshake) => GateSummary {
                proceeded: true,
                response_status: None,
                room: handshake.path_param("room").map(str::to_string),
            },
            UpgradeGateOutcome::ShortCircuit(response) => GateSummary {
                proceeded: false,
                response_status: Some(response.into_http().status().as_u16()),
                room: None,
            },
        }
    }

    async fn summarize_gate(middleware: Vec<Arc<dyn HttpMiddleware>>) -> GateSummary {
        summarize(run_upgrade_gate(&middleware, &no_forward_targets(), handshake()).await)
    }

    #[tokio::test]
    async fn proceeds_when_every_middleware_delegates() {
        let middleware: Vec<Arc<dyn HttpMiddleware>> =
            vec![Arc::new(PassThrough), Arc::new(PassThrough)];

        assert!(summarize_gate(middleware).await.proceeded);
    }

    #[tokio::test]
    async fn proceeds_when_there_is_no_middleware() {
        assert!(summarize_gate(Vec::new()).await.proceeded);
    }

    #[tokio::test]
    async fn hands_the_handshake_back_untouched_when_it_proceeds() {
        let intact = Request::new(Method::GET, "/chat".to_string())
            .with_path_params(HashMap::from([("room".to_string(), "7".to_string())]));
        let summary = summarize(
            run_upgrade_gate(
                &[Arc::new(PassThrough) as Arc<dyn HttpMiddleware>],
                &no_forward_targets(),
                intact,
            )
            .await,
        );

        assert!(summary.proceeded);
        assert_eq!(summary.room, Some("7".to_string()));
    }

    #[tokio::test]
    async fn short_circuits_when_a_middleware_returns_a_response() {
        let summary = summarize_gate(vec![Arc::new(RespondsWith { status: 403 })]).await;

        assert!(!summary.proceeded);
        assert_eq!(summary.response_status, Some(403));
    }

    #[tokio::test]
    async fn short_circuits_when_a_middleware_redirects() {
        assert_eq!(
            summarize_gate(vec![Arc::new(RedirectsAway)])
                .await
                .response_status,
            Some(303),
        );
    }

    #[tokio::test]
    async fn resolves_a_forward_from_a_middleware() {
        let middleware: Vec<Arc<dyn HttpMiddleware>> = vec![Arc::new(ForwardsTo { name: "target" })];
        let forward_targets =
            Arc::new(ForwardTargets::new(vec![NamedHandler::new("target", Arc::new(Target))]));
        let summary = summarize(run_upgrade_gate(&middleware, &forward_targets, handshake()).await);

        assert!(!summary.proceeded);
        assert_eq!(summary.response_status, Some(222));
    }

    #[tokio::test]
    async fn applies_the_first_declared_middleware_outermost() {
        assert_eq!(
            summarize_gate(vec![
                Arc::new(RespondsWith { status: 401 }),
                Arc::new(RespondsWith { status: 403 }),
            ])
            .await
            .response_status,
            Some(401),
        );
    }

    #[tokio::test]
    async fn proceeds_when_a_middleware_overrides_after_delegating() {
        assert!(
            summarize_gate(vec![Arc::new(OverridesAfterDelegating)])
                .await
                .proceeded
        );
    }
}
