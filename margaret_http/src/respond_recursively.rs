use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::request::Request;
use crate::resolve_continuation::resolve_continuation;
use crate::response::Response;

pub(crate) async fn respond_recursively(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    first: Arc<dyn Handler>,
) -> Response {
    let outcome = first.handle(&request).await;

    resolve_continuation(forward_targets, request, outcome).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use async_trait::async_trait;
    use http::Method;
    use http_body_util::BodyExt;

    use super::respond_recursively;
    use crate::forward::Forward;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::named_handler::NamedHandler;
    use crate::redirect::Redirect;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

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

    struct RedirectingResponder;

    #[async_trait]
    impl Handler for RedirectingResponder {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Redirect::see_other("http://localhost/greeting".to_string()))
        }
    }

    fn forward_targets_with(targets: Vec<NamedHandler>) -> Arc<ForwardTargets> {
        Arc::new(ForwardTargets::new(targets))
    }

    async fn status_of(forward_targets: Arc<ForwardTargets>, first: Arc<dyn Handler>) -> u16 {
        respond_recursively(
            &forward_targets,
            Request::new(Method::GET, "/".to_string()),
            first,
        )
        .await
        .into_http()
        .status()
        .as_u16()
    }

    #[tokio::test]
    async fn forwards_to_the_target_responder() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("target", Arc::new(Target))]);

        assert_eq!(
            status_of(forward_targets, Arc::new(ForwardToTarget)).await,
            222
        );
    }

    #[tokio::test]
    async fn returns_a_server_error_on_a_forward_cycle() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("origin", Arc::new(ForwardToSelf))]);

        assert_eq!(
            status_of(forward_targets, Arc::new(ForwardToSelf)).await,
            500
        );
    }

    #[tokio::test]
    async fn returns_a_server_error_when_forwarding_to_an_unregistered_name() {
        assert_eq!(
            status_of(forward_targets_with(Vec::new()), Arc::new(ForwardToUnknown)).await,
            500
        );
    }

    #[tokio::test]
    async fn returns_a_redirect_response() {
        let response = respond_recursively(
            &forward_targets_with(Vec::new()),
            Request::new(Method::GET, "/".to_string()),
            Arc::new(RedirectingResponder),
        )
        .await
        .into_http();

        assert_eq!(response.status().as_u16(), 303);
        assert_eq!(
            response
                .headers()
                .get("location")
                .expect("the location header is present"),
            "http://localhost/greeting"
        );
    }

    #[tokio::test]
    async fn installs_forwarded_path_parameters_for_the_target() {
        let forward_targets =
            forward_targets_with(vec![NamedHandler::new("article", Arc::new(EchoArticle))]);
        let response = respond_recursively(
            &forward_targets,
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
