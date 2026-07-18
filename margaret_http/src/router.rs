use std::collections::HashMap;
use std::sync::Arc;

use matchit::InsertError;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::method_handler::MethodHandler;
use crate::request::Request;
use crate::respond_recursively::respond_recursively;
use crate::response::Response;
use crate::route_entry::RouteEntry;

type PathHandlers = HashMap<&'static str, Arc<dyn Handler>>;

pub struct Router {
    matcher: matchit::Router<PathHandlers>,
}

impl Router {
    pub fn build(entries: Vec<RouteEntry>) -> Result<Self, InsertError> {
        let mut matcher: matchit::Router<PathHandlers> = matchit::Router::new();

        for RouteEntry { handlers, path } in entries {
            matcher.insert(
                path,
                handlers
                    .into_iter()
                    .map(|MethodHandler { handler, method }| (method, handler))
                    .collect(),
            )?;
        }

        Ok(Self { matcher })
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

        let Some(handler) = matched.value.get(method).cloned() else {
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
    use crate::method_handler::MethodHandler;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::route_entry::RouteEntry;

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
        super::Router::build(vec![
            RouteEntry::new(
                "/items",
                vec![
                    MethodHandler::new("GET", Arc::new(PlainOk)),
                    MethodHandler::new("POST", Arc::new(PlainOk)),
                ],
            ),
            RouteEntry::new(
                "/items/{id}",
                vec![MethodHandler::new("GET", Arc::new(EchoId))],
            ),
        ])
        .expect("the route entries register cleanly")
    }

    #[test]
    fn reports_conflicting_route_paths_instead_of_panicking() {
        let conflict = super::Router::build(vec![
            RouteEntry::new(
                "/items/{id}",
                vec![MethodHandler::new("GET", Arc::new(PlainOk))],
            ),
            RouteEntry::new(
                "/items/{name}",
                vec![MethodHandler::new("GET", Arc::new(PlainOk))],
            ),
        ]);

        assert!(conflict.is_err());
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
