use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::Handler;
use crate::method::Method;
use crate::request::Request;
use crate::respond_recursively::respond_recursively;
use crate::response::Response;

pub struct Router {
    by_name: HashMap<&'static str, Arc<dyn Handler>>,
    matcher: matchit::Router<usize>,
    paths: HashMap<String, usize>,
    routes: Vec<HashMap<Method, Arc<dyn Handler>>>,
}

impl Router {
    pub fn empty() -> Self {
        Self {
            by_name: HashMap::new(),
            matcher: matchit::Router::new(),
            paths: HashMap::new(),
            routes: Vec::new(),
        }
    }

    pub fn route(mut self, method: Method, path: &str, handler: Arc<dyn Handler>) -> Self {
        self.register(method, path, handler);

        self
    }

    pub fn route_with_name(
        mut self,
        method: Method,
        path: &str,
        name: &'static str,
        handler: Arc<dyn Handler>,
    ) -> Self {
        let handler = self.register(method, path, handler);

        self.by_name.insert(name, handler);

        self
    }

    pub async fn respond(&self, request: Request) -> Response {
        let path = request.server().path().to_string();
        let method = request.server().method();

        let matched = match self.matcher.at(&path) {
            Ok(matched) => matched,
            Err(matchit::MatchError::NotFound) => return Response::not_found(),
        };

        let Some(handler) = self.routes[*matched.value].get(&method).cloned() else {
            return Response::text(405, "Method Not Allowed");
        };

        let path_params = matched
            .params
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        let request = request.with_path_params(path_params);

        respond_recursively(&self.by_name, request, handler).await
    }

    fn register(
        &mut self,
        method: Method,
        path: &str,
        handler: Arc<dyn Handler>,
    ) -> Arc<dyn Handler> {
        let index = match self.paths.get(path) {
            Some(&index) => index,
            None => {
                let index = self.routes.len();

                self.routes.push(HashMap::new());
                self.matcher
                    .insert(path.to_string(), index)
                    .expect("a well-formed route path");
                self.paths.insert(path.to_string(), index);

                index
            }
        };

        self.routes[index].insert(method, handler.clone());

        handler
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::Router;
    use crate::forward::Forward;
    use crate::handler::Handler;
    use crate::method::Method;
    use crate::request::Request;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

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

    fn router() -> Router {
        Router::empty()
            .route(Method::Get, "/items", Arc::new(PlainOk))
            .route(Method::Post, "/items", Arc::new(PlainOk))
            .route(Method::Get, "/items/{id}", Arc::new(EchoId))
    }

    async fn status_of(method: Method, path: &str) -> u16 {
        router()
            .respond(Request::new(method, path.to_string()))
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn routes_a_matching_request() {
        assert_eq!(status_of(Method::Get, "/items").await, 200);
    }

    #[tokio::test]
    async fn extracts_path_parameters_for_the_handler() {
        assert_eq!(status_of(Method::Get, "/items/7").await, 200);
    }

    #[tokio::test]
    async fn returns_404_for_an_unknown_path() {
        assert_eq!(status_of(Method::Get, "/missing").await, 404);
    }

    #[tokio::test]
    async fn returns_405_for_an_unregistered_method() {
        assert_eq!(status_of(Method::Delete, "/items").await, 405);
    }

    struct ForwardsToPublicGreeting;

    #[async_trait]
    impl Handler for ForwardsToPublicGreeting {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::from(Forward::to(
                "crate::routes::public::get_greeting::GetGreeting",
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

    async fn handle_status(router: &Router, method: Method, path: &str) -> u16 {
        router
            .respond(Request::new(method, path.to_string()))
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn a_server_cannot_forward_to_a_route_registered_on_another_server() {
        let public_server = Router::empty().route_with_name(
            Method::Get,
            "/greeting",
            "crate::routes::public::get_greeting::GetGreeting",
            Arc::new(PlainOk),
        );
        let internal_server =
            Router::empty().route(Method::Get, "/forward", Arc::new(ForwardsToPublicGreeting));

        assert_eq!(
            handle_status(&public_server, Method::Get, "/greeting").await,
            200
        );
        assert_eq!(
            handle_status(&internal_server, Method::Get, "/forward").await,
            404
        );
    }
}
