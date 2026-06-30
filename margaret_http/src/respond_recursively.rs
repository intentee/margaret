use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use crate::handler::Handler;
use crate::request::Request;
use crate::responded::Responded;
use crate::response::Response;

pub(crate) async fn respond_recursively(
    by_name: &HashMap<&'static str, Arc<dyn Handler>>,
    request: Request,
    first: Arc<dyn Handler>,
) -> Response {
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut request = request;
    let mut outcome = first.handle(request.clone()).await;

    loop {
        match outcome {
            Responded::Done(response) => return response,
            Responded::Forward(forward) => {
                if !visited.insert(forward.name()) {
                    return Response::text(500, "Internal Server Error");
                }

                let Some(target) = by_name.get(forward.name()).cloned() else {
                    return Response::not_found();
                };

                request.clear_path_params();
                outcome = target.handle(request.clone()).await;
            }
            Responded::Intercept(interception) => {
                return interception.render(&request).await;
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
    use crate::responded::Responded;
    use crate::response::Response;

    struct ForwardToTarget;

    #[async_trait]
    impl Handler for ForwardToTarget {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to("target"))
        }
    }

    struct ForwardToSelf;

    #[async_trait]
    impl Handler for ForwardToSelf {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to("origin"))
        }
    }

    struct ForwardToUnknown;

    #[async_trait]
    impl Handler for ForwardToUnknown {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to("unknown"))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::Done(Response::text(222, "target"))
        }
    }

    struct Rendering;

    #[async_trait]
    impl DeferredInterception for Rendering {
        async fn render(self: Box<Self>, _request: &Request) -> Response {
            Response::text(244, "rendered")
        }
    }

    struct Intercepting;

    #[async_trait]
    impl Handler for Intercepting {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::Intercept(Box::new(Rendering))
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
    async fn applies_an_interception_after_the_response_is_final() {
        assert_eq!(status_of(HashMap::new(), Arc::new(Intercepting)).await, 244);
    }
}
