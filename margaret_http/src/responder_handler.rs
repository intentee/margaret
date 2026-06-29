use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::request::Request;
use crate::response::Response;

struct FnHandler<Responder, Extract> {
    extract: Extract,
    responder: Arc<Responder>,
}

#[async_trait]
impl<Responder, Extract, ResponseFuture> Handler for FnHandler<Responder, Extract>
where
    Responder: Send + Sync + 'static,
    Extract: Fn(Arc<Responder>, Request) -> ResponseFuture + Send + Sync + 'static,
    ResponseFuture: Future<Output = Response> + Send + 'static,
{
    async fn handle(&self, request: Request) -> Response {
        (self.extract)(self.responder.clone(), request).await
    }
}

pub fn responder_handler<Responder, Extract, ResponseFuture>(
    responder: Arc<Responder>,
    extract: Extract,
) -> Arc<dyn Handler>
where
    Responder: Send + Sync + 'static,
    Extract: Fn(Arc<Responder>, Request) -> ResponseFuture + Send + Sync + 'static,
    ResponseFuture: Future<Output = Response> + Send + 'static,
{
    Arc::new(FnHandler { extract, responder })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::responder_handler;
    use crate::method::Method;
    use crate::request::Request;
    use crate::response::Response;

    struct Echo;

    impl Echo {
        async fn respond(&self, id: String) -> Response {
            Response::text(200, id)
        }
    }

    #[tokio::test]
    async fn injects_request_values_into_the_responder() {
        let handler = responder_handler(
            Arc::new(Echo),
            |responder: Arc<Echo>, request: Request| async move {
                responder
                    .respond(
                        request
                            .path_param("id")
                            .expect("the test request carries the id parameter")
                            .to_string(),
                    )
                    .await
            },
        );
        let mut request = Request::new(Method::Get, "/echo/7".to_string());
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "7".to_string());
        request.set_path_params(path_params);

        let response = handler.handle(request).await;

        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "7");
    }
}
