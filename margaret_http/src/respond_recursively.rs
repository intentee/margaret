use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn respond_recursively(
    by_name: &HashMap<&'static str, Arc<dyn Handler>>,
    request: Request,
    first: Arc<dyn Handler>,
) -> Response {
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut request = request;
    let mut outcome = first.handle(&request).await;

    loop {
        match outcome {
            ResponseContinuation::Done(response) => return response,
            ResponseContinuation::Forward(forward) => {
                if !visited.insert(forward.name()) {
                    return Response::text(500, "Internal Server Error");
                }

                let Some(target) = by_name.get(forward.name()).cloned() else {
                    return Response::not_found();
                };

                request = request.with_path_params(HashMap::new());
                outcome = target.handle(&request).await;
            }
            ResponseContinuation::Intercept(interception) => {
                outcome = interception.render(&request).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::respond_recursively;
    use crate::deferred_interception::DeferredInterception;
    use crate::forward::Forward;
    use crate::handler::Handler;
    use crate::method::Method;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct ForwardToTarget;

    #[async_trait]
    impl Handler for ForwardToTarget {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::to("target"))
        }
    }

    struct ForwardToSelf;

    #[async_trait]
    impl Handler for ForwardToSelf {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::to("origin"))
        }
    }

    struct ForwardToUnknown;

    #[async_trait]
    impl Handler for ForwardToUnknown {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::to("unknown"))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(222, "target"))
        }
    }

    struct ForwardingInterception;

    #[async_trait]
    impl DeferredInterception for ForwardingInterception {
        async fn render(self: Box<Self>, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::to("target"))
        }
    }

    struct InterceptsToForward;

    #[async_trait]
    impl Handler for InterceptsToForward {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Intercept(Box::new(ForwardingInterception))
        }
    }

    async fn status_of(
        by_name: HashMap<&'static str, Arc<dyn Handler>>,
        first: Arc<dyn Handler>,
    ) -> u16 {
        respond_recursively(&by_name, Request::new(Method::Get, "/".to_string()), first)
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn forwards_to_the_target_responder() {
        let mut by_name: HashMap<&'static str, Arc<dyn Handler>> = HashMap::new();

        by_name.insert("target", Arc::new(Target));

        assert_eq!(status_of(by_name, Arc::new(ForwardToTarget)).await, 222);
    }

    #[tokio::test]
    async fn returns_a_server_error_on_a_forward_cycle() {
        let mut by_name: HashMap<&'static str, Arc<dyn Handler>> = HashMap::new();

        by_name.insert("origin", Arc::new(ForwardToSelf));

        assert_eq!(status_of(by_name, Arc::new(ForwardToSelf)).await, 500);
    }

    #[tokio::test]
    async fn returns_not_found_when_forwarding_to_an_unknown_name() {
        assert_eq!(
            status_of(HashMap::new(), Arc::new(ForwardToUnknown)).await,
            404
        );
    }

    #[tokio::test]
    async fn repeats_the_stack_when_an_interceptor_returns_a_responder() {
        let mut by_name: HashMap<&'static str, Arc<dyn Handler>> = HashMap::new();

        by_name.insert("target", Arc::new(Target));

        assert_eq!(status_of(by_name, Arc::new(InterceptsToForward)).await, 222);
    }
}
