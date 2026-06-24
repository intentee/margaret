use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;

pub struct Router {
    matcher: matchit::Router<usize>,
    paths: HashMap<String, usize>,
    routes: Vec<HashMap<Method, Arc<dyn Handler>>>,
}

impl Router {
    pub fn empty() -> Self {
        Self {
            matcher: matchit::Router::new(),
            paths: HashMap::new(),
            routes: Vec::new(),
        }
    }

    pub fn route(mut self, method: Method, path: &str, handler: Arc<dyn Handler>) -> Self {
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

        self.routes[index].insert(method, handler);

        self
    }
}

#[async_trait]
impl Handler for Router {
    async fn handle(&self, mut request: Request) -> Response {
        let path = request.path().to_string();

        match self.matcher.at(&path) {
            Ok(matched) => match self.routes[*matched.value].get(&request.method()).cloned() {
                Some(handler) => {
                    let path_params = matched
                        .params
                        .iter()
                        .map(|(name, value)| (name.to_string(), value.to_string()))
                        .collect();

                    request.set_path_params(path_params);

                    handler.handle(request).await
                }
                None => Response::text(405, "Method Not Allowed"),
            },
            Err(_) => Response::text(404, "Not Found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::Router;
    use crate::handler::Handler;
    use crate::http_responder::HttpResponder;
    use crate::method::Method;
    use crate::request::Request;
    use crate::responder_handler::responder_handler;
    use crate::response::Response;

    struct Ok200;

    #[async_trait]
    impl HttpResponder for Ok200 {
        async fn respond(&self, request: Request) -> Response {
            Response::text(200, request.path_param("id").unwrap_or("none").to_string())
        }
    }

    fn router() -> Router {
        Router::empty()
            .route(Method::Get, "/items", responder_handler(Arc::new(Ok200)))
            .route(Method::Post, "/items", responder_handler(Arc::new(Ok200)))
            .route(
                Method::Get,
                "/items/{id}",
                responder_handler(Arc::new(Ok200)),
            )
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
