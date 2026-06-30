use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::method::Method;
use crate::request::Request;
use crate::respond_recursively::respond_recursively;
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

        Responded::Done(respond_recursively(&self.by_symbol, request, handler).await)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::Router;
    use crate::handler::Handler;
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
}
