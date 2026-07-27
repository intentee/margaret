use std::sync::Arc;

use async_trait::async_trait;

use crate::access_decision::AccessDecision;
use crate::access_policy::AccessPolicy;
use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct AccessControlledHandler<Policy> {
    next: Arc<dyn Handler>,
    policy: Arc<Policy>,
}

#[async_trait]
impl<Policy> Handler for AccessControlledHandler<Policy>
where
    Policy: AccessPolicy + 'static,
{
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        match self
            .policy
            .decide(request)
            .await
            .map_err(HandlerError::consumer)?
        {
            AccessDecision::Allowed => self.next.handle(request).await,
            AccessDecision::Denied(response) => Ok(ResponseContinuation::from(response)),
        }
    }
}

pub fn access_control<Policy>(policy: Arc<Policy>, next: Arc<dyn Handler>) -> Arc<dyn Handler>
where
    Policy: AccessPolicy + 'static,
{
    Arc::new(AccessControlledHandler { next, policy })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use async_trait::async_trait;
    use http::Method;
    use http_body_util::BodyExt;

    use super::access_control;
    use crate::access_decision::AccessDecision;
    use crate::access_policy::AccessPolicy;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::handler_error::HandlerError;
    use crate::request::Request;
    use crate::respond_recursively::respond_recursively;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct Deny;

    struct Allow;

    #[async_trait]
    impl AccessPolicy for Allow {
        async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
            Ok(AccessDecision::Allowed)
        }
    }

    #[async_trait]
    impl AccessPolicy for Deny {
        async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
            Ok(AccessDecision::Denied(Response::forbidden()))
        }
    }

    struct Fail;

    #[async_trait]
    impl AccessPolicy for Fail {
        async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
            Err(anyhow::anyhow!("policy storage failed"))
        }
    }

    struct Probe {
        called: Arc<AtomicBool>,
    }

    #[async_trait]
    impl Handler for Probe {
        async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
            self.called.store(true, Ordering::SeqCst);

            Ok(ResponseContinuation::from(Response::text(200, "unsafe")))
        }
    }

    async fn response(
        policy: impl AccessPolicy + 'static,
        called: Arc<AtomicBool>,
    ) -> http::Response<http_body_util::Full<bytes::Bytes>> {
        let handler = access_control(policy.into(), Arc::new(Probe { called }));

        respond_recursively(
            &Arc::new(ForwardTargets::new(Vec::new())),
            Request::new(Method::GET, "/private".to_string()),
            handler,
        )
        .await
        .into_http()
    }

    #[tokio::test]
    async fn denial_does_not_execute_the_handler() {
        let called = Arc::new(AtomicBool::new(false));
        let response = response(Deny, called.clone()).await;

        assert_eq!(response.status(), 403);
        assert!(!called.load(Ordering::SeqCst));
        assert_eq!(
            response
                .into_body()
                .collect()
                .await
                .expect("the denial body is collected")
                .to_bytes(),
            "Forbidden"
        );
    }

    #[tokio::test]
    async fn permission_executes_the_handler() {
        let called = Arc::new(AtomicBool::new(false));
        let response = response(Allow, called.clone()).await;

        assert_eq!(response.status(), 200);
        assert!(called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn policy_system_errors_do_not_execute_the_handler() {
        let called = Arc::new(AtomicBool::new(false));
        let response = response(Fail, called.clone()).await;

        assert_eq!(response.status(), 500);
        assert!(!called.load(Ordering::SeqCst));
    }
}
