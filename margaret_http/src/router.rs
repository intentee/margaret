use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::method::Method;
use crate::request::Request;
use crate::responded::Responded;
use crate::response::Response;

pub struct Router {
    by_symbol: HashMap<&'static str, Arc<dyn Handler>>,
    matcher: matchit::Router<usize>,
    paths: HashMap<String, usize>,
    routes: Vec<HashMap<Method, Arc<dyn Handler>>>,
}

impl Router {
    pub fn empty() -> Self {
        Self {
            by_symbol: HashMap::new(),
            matcher: matchit::Router::new(),
            paths: HashMap::new(),
            routes: Vec::new(),
        }
    }

    pub fn route(
        mut self,
        method: Method,
        path: &str,
        symbol_key: &'static str,
        handler: Arc<dyn Handler>,
    ) -> Self {
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
        self.by_symbol.insert(symbol_key, handler);

        self
    }

    async fn drive(&self, request: Request, first: Arc<dyn Handler>) -> Response {
        let mut visited: HashSet<&'static str> = HashSet::new();
        let mut request = request;
        let mut outcome = first.handle(request.clone()).await;

        loop {
            match outcome {
                Responded::Done(response) => return response,
                Responded::Forward(forward) => {
                    if !visited.insert(forward.key()) {
                        return Response::text(500, "Internal Server Error");
                    }

                    let Some(target) = self.by_symbol.get(forward.key()).cloned() else {
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
}

#[async_trait]
impl Handler for Router {
    async fn handle(&self, mut request: Request) -> Responded {
        let path = request.path().to_string();

        let handler = match self.matcher.at(&path) {
            Ok(matched) => match self.routes[*matched.value].get(&request.method()).cloned() {
                Some(handler) => {
                    let path_params = matched
                        .params
                        .iter()
                        .map(|(name, value)| (name.to_string(), value.to_string()))
                        .collect();

                    request.set_path_params(path_params);

                    handler
                }
                None => return Responded::Done(Response::text(405, "Method Not Allowed")),
            },
            Err(_) => return Responded::Done(Response::not_found()),
        };

        Responded::Done(self.drive(request, handler).await)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::Router;
    use crate::forward::Forward;
    use crate::handler::Handler;
    use crate::http_route_symbol::HttpRouteSymbol;
    use crate::method::Method;
    use crate::request::Request;
    use crate::responded::Responded;
    use crate::response::Response;

    struct EchoId;

    #[async_trait]
    impl Handler for EchoId {
        async fn handle(&self, request: Request) -> Responded {
            Responded::Done(Response::text(
                200,
                request.path_param("id").unwrap_or("none").to_string(),
            ))
        }
    }

    struct SymbolTarget;

    impl HttpRouteSymbol for SymbolTarget {
        fn route_key(&self) -> &'static str {
            "target"
        }
    }

    struct SymbolSelf;

    impl HttpRouteSymbol for SymbolSelf {
        fn route_key(&self) -> &'static str {
            "origin"
        }
    }

    struct SymbolUnknown;

    impl HttpRouteSymbol for SymbolUnknown {
        fn route_key(&self) -> &'static str {
            "unknown"
        }
    }

    struct ForwardToTarget;

    #[async_trait]
    impl Handler for ForwardToTarget {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to(SymbolTarget))
        }
    }

    struct ForwardToSelf;

    #[async_trait]
    impl Handler for ForwardToSelf {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to(SymbolSelf))
        }
    }

    struct ForwardToUnknown;

    #[async_trait]
    impl Handler for ForwardToUnknown {
        async fn handle(&self, _request: Request) -> Responded {
            Responded::from(Forward::to(SymbolUnknown))
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
    impl crate::deferred_interception::DeferredInterception for Rendering {
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

    fn router() -> Router {
        Router::empty()
            .route(Method::Get, "/items", "items", Arc::new(EchoId))
            .route(Method::Post, "/items", "items", Arc::new(EchoId))
            .route(Method::Get, "/items/{id}", "item", Arc::new(EchoId))
    }

    async fn status_of(method: Method, path: &str) -> u16 {
        router()
            .handle(Request::new(method, path.to_string()))
            .await
            .into_http()
            .status()
            .as_u16()
    }

    async fn status_for(router: Router, path: &str) -> u16 {
        router
            .handle(Request::new(Method::Get, path.to_string()))
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

    #[tokio::test]
    async fn forwards_to_the_target_responder() {
        let router = Router::empty()
            .route(Method::Get, "/origin", "origin", Arc::new(ForwardToTarget))
            .route(Method::Get, "/target", "target", Arc::new(Target));

        assert_eq!(status_for(router, "/origin").await, 222);
    }

    #[tokio::test]
    async fn returns_a_server_error_on_a_forward_cycle() {
        let router =
            Router::empty().route(Method::Get, "/origin", "origin", Arc::new(ForwardToSelf));

        assert_eq!(status_for(router, "/origin").await, 500);
    }

    #[tokio::test]
    async fn returns_not_found_when_forwarding_to_an_unknown_symbol() {
        let router =
            Router::empty().route(Method::Get, "/origin", "origin", Arc::new(ForwardToUnknown));

        assert_eq!(status_for(router, "/origin").await, 404);
    }

    #[tokio::test]
    async fn applies_an_interception_after_the_response_is_final() {
        let router = Router::empty().route(Method::Get, "/page", "page", Arc::new(Intercepting));

        assert_eq!(status_for(router, "/page").await, 244);
    }
}
