use std::sync::Arc;

use async_trait::async_trait;

use crate::access_decision::AccessDecision;
use crate::access_policy::AccessPolicy;
use crate::handler_error::HandlerError;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct PolicyMiddleware<Policy> {
    policy: Arc<Policy>,
}

#[async_trait]
impl<Policy> HttpMiddleware for PolicyMiddleware<Policy>
where
    Policy: AccessPolicy + 'static,
{
    async fn process(
        &self,
        request: &Request,
        next: Next,
    ) -> Result<ResponseContinuation, HandlerError> {
        match self
            .policy
            .decide(request)
            .await
            .map_err(HandlerError::consumer)?
        {
            AccessDecision::Allowed => next.run(request).await,
            AccessDecision::Denied(response) => Ok(ResponseContinuation::from(response)),
        }
    }
}

pub fn access_policy_middleware<Policy>(
    policy: Arc<Policy>,
    mut middleware: Vec<Arc<dyn HttpMiddleware>>,
) -> Vec<Arc<dyn HttpMiddleware>>
where
    Policy: AccessPolicy + 'static,
{
    middleware.insert(0, Arc::new(PolicyMiddleware { policy }));

    middleware
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering;

    use async_trait::async_trait;
    use http::Method;

    use super::access_policy_middleware;
    use crate::access_decision::AccessDecision;
    use crate::access_policy::AccessPolicy;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct Allow;

    #[async_trait]
    impl AccessPolicy for Allow {
        async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
            Ok(AccessDecision::Allowed)
        }
    }

    #[test]
    fn places_the_policy_before_application_middleware() {
        let middleware = access_policy_middleware(Arc::new(Allow), Vec::new());

        assert_eq!(middleware.len(), 1);
    }

    #[tokio::test]
    async fn denial_is_an_expected_outcome() {
        struct Deny;

        #[async_trait]
        impl AccessPolicy for Deny {
            async fn decide(&self, _request: &Request) -> anyhow::Result<AccessDecision> {
                Ok(AccessDecision::Denied(Response::forbidden()))
            }
        }

        let middleware = access_policy_middleware(Arc::new(Deny), Vec::new());
        let request = Request::new(Method::GET, "/socket".to_string());
        let called = Arc::new(AtomicBool::new(false));
        let result = middleware[0]
            .process(
                &request,
                crate::next::Next::one_shot(Box::new(Probe {
                    called: called.clone(),
                })),
            )
            .await
            .expect("denial is not a system error");

        assert_eq!(
            std::mem::discriminant(&result),
            std::mem::discriminant(&ResponseContinuation::from(Response::forbidden()))
        );
        assert!(!called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn permission_delegates_to_the_next_handler() {
        let middleware = access_policy_middleware(Arc::new(Allow), Vec::new());
        let request = Request::new(Method::GET, "/socket".to_string());
        let called = Arc::new(AtomicBool::new(false));
        let result = middleware[0]
            .process(
                &request,
                crate::next::Next::one_shot(Box::new(Probe {
                    called: called.clone(),
                })),
            )
            .await
            .expect("permission delegates without a system error");

        assert_eq!(
            std::mem::discriminant(&result),
            std::mem::discriminant(&ResponseContinuation::from(Response::forbidden()))
        );
        assert!(called.load(Ordering::SeqCst));
    }

    struct Probe {
        called: Arc<AtomicBool>,
    }

    #[async_trait]
    impl crate::one_shot_handler::OneShotHandler for Probe {
        async fn handle(
            self: Box<Self>,
            _request: &Request,
        ) -> Result<
            crate::response_continuation::ResponseContinuation,
            crate::handler_error::HandlerError,
        > {
            self.called.store(true, Ordering::SeqCst);

            Ok(ResponseContinuation::from(Response::text(200, "allowed")))
        }
    }
}
