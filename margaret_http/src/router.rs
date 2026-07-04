use std::collections::HashMap;
use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::request::Request;
use crate::respond_recursively::respond_recursively;
use crate::response::Response;

pub struct Router {
    matcher: matchit::Router<usize>,
    routes: Vec<HashMap<&'static str, Arc<dyn Handler>>>,
}

impl Router {
    pub(crate) fn new(
        matcher: matchit::Router<usize>,
        routes: Vec<HashMap<&'static str, Arc<dyn Handler>>>,
    ) -> Self {
        Self { matcher, routes }
    }

    pub(crate) async fn respond(
        &self,
        request: Request,
        forward_targets: &Arc<ForwardTargets>,
    ) -> Response {
        let path = request.inputs.server.path().to_string();
        let method = request.inputs.server.method();

        let matched = match self.matcher.at(&path) {
            Ok(matched) => matched,
            Err(matchit::MatchError::NotFound) => return Response::not_found(),
        };

        let Some(handler) = self.routes[*matched.value].get(method).cloned() else {
            return Response::text(405, "Method Not Allowed");
        };

        let path_params = matched
            .params
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let request = request.with_path_params(path_params);

        respond_recursively(forward_targets, request, handler).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use http::Method;

    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::router_builder::RouterBuilder;

    struct EchoId;

    #[async_trait]
    impl Handler for EchoId {
        async fn handle(&self, request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(
                200,
                request
                    .path_param("id")
                    .expect("the matched route binds the id path parameter")
                    .to_string(),
            ))
        }
    }

    struct PlainOk;

    #[async_trait]
    impl Handler for PlainOk {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(200, "ok"))
        }
    }

    fn router() -> super::Router {
        RouterBuilder::empty()
            .route("GET", "/items", Arc::new(PlainOk))
            .route("POST", "/items", Arc::new(PlainOk))
            .route("GET", "/items/{id}", Arc::new(EchoId))
            .build()
    }

    async fn status_of(method: Method, path: &str) -> u16 {
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));

        router()
            .respond(Request::new(method, path.to_string()), &forward_targets)
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn routes_a_matching_request() {
        assert_eq!(status_of(Method::GET, "/items").await, 200);
    }

    #[tokio::test]
    async fn extracts_path_parameters_for_the_handler() {
        assert_eq!(status_of(Method::GET, "/items/7").await, 200);
    }

    #[tokio::test]
    async fn returns_404_for_an_unknown_path() {
        assert_eq!(status_of(Method::GET, "/missing").await, 404);
    }

    #[tokio::test]
    async fn returns_405_for_an_unregistered_method() {
        assert_eq!(status_of(Method::DELETE, "/items").await, 405);
    }
}
