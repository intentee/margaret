use std::collections::HashSet;
use std::sync::Arc;

use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;
use crate::servers::Servers;

pub(crate) async fn respond_recursively(
    servers: &Arc<Servers>,
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
                let name = forward.name();

                if !visited.insert(name) {
                    return Response::text(500, "Internal Server Error");
                }

                let Some(target) = servers.forward_target(name) else {
                    return Response::not_found();
                };

                request = request.with_path_params(forward.into_path_params());
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
    use http::Method;
    use http_body_util::BodyExt;

    use super::respond_recursively;
    use crate::deferred_interception::DeferredInterception;
    use crate::forward::Forward;
    use crate::handler::Handler;
    use crate::named_handler::NamedHandler;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::servers::Servers;

    struct ForwardToTarget;

    #[async_trait]
    impl Handler for ForwardToTarget {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("target", HashMap::new()))
        }
    }

    struct ForwardToSelf;

    #[async_trait]
    impl Handler for ForwardToSelf {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("origin", HashMap::new()))
        }
    }

    struct ForwardToUnknown;

    #[async_trait]
    impl Handler for ForwardToUnknown {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("unknown", HashMap::new()))
        }
    }

    struct Target;

    #[async_trait]
    impl Handler for Target {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(222, "target"))
        }
    }

    struct ForwardToArticle;

    #[async_trait]
    impl Handler for ForwardToArticle {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new(
                "article",
                HashMap::from([("article".to_string(), "7".to_string())]),
            ))
        }
    }

    struct EchoArticle;

    #[async_trait]
    impl Handler for EchoArticle {
        async fn handle(&self, request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(
                200,
                request
                    .path_param("article")
                    .expect("the forward supplies the article path parameter")
                    .to_string(),
            ))
        }
    }

    struct ForwardingInterception;

    #[async_trait]
    impl DeferredInterception for ForwardingInterception {
        async fn render(self: Box<Self>, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::new("target", HashMap::new()))
        }
    }

    struct InterceptsToForward;

    #[async_trait]
    impl Handler for InterceptsToForward {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Intercept(Box::new(ForwardingInterception))
        }
    }

    fn servers_with(targets: Vec<NamedHandler>) -> Arc<Servers> {
        Arc::new(Servers::new(Vec::new(), targets))
    }

    async fn status_of(servers: Arc<Servers>, first: Arc<dyn Handler>) -> u16 {
        respond_recursively(&servers, Request::new(Method::GET, "/".to_string()), first)
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn forwards_to_the_target_responder() {
        let servers = servers_with(vec![NamedHandler::new("target", Arc::new(Target))]);

        assert_eq!(status_of(servers, Arc::new(ForwardToTarget)).await, 222);
    }

    #[tokio::test]
    async fn returns_a_server_error_on_a_forward_cycle() {
        let servers = servers_with(vec![NamedHandler::new("origin", Arc::new(ForwardToSelf))]);

        assert_eq!(status_of(servers, Arc::new(ForwardToSelf)).await, 500);
    }

    #[tokio::test]
    async fn returns_not_found_when_forwarding_to_an_unknown_name() {
        assert_eq!(
            status_of(servers_with(Vec::new()), Arc::new(ForwardToUnknown)).await,
            404
        );
    }

    #[tokio::test]
    async fn repeats_the_stack_when_an_interceptor_returns_a_responder() {
        let servers = servers_with(vec![NamedHandler::new("target", Arc::new(Target))]);

        assert_eq!(status_of(servers, Arc::new(InterceptsToForward)).await, 222);
    }

    #[tokio::test]
    async fn installs_forwarded_path_parameters_for_the_target() {
        let servers = servers_with(vec![NamedHandler::new("article", Arc::new(EchoArticle))]);
        let response = respond_recursively(
            &servers,
            Request::new(Method::GET, "/".to_string()),
            Arc::new(ForwardToArticle),
        )
        .await
        .into_http();

        assert_eq!(response.status().as_u16(), 200);
        assert_eq!(
            response
                .into_body()
                .collect()
                .await
                .expect("the response body collects")
                .to_bytes(),
            "7"
        );
    }
}
