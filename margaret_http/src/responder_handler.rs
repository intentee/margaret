use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::request::Request;
use crate::responded::Responded;

struct FnHandler<Responder, Extract> {
    extract: Extract,
    responder: Arc<Responder>,
}

#[async_trait]
impl<Responder, Extract, OutcomeFuture> Handler for FnHandler<Responder, Extract>
where
    Responder: Send + Sync + 'static,
    Extract: Fn(Arc<Responder>, Request) -> OutcomeFuture + Send + Sync + 'static,
    OutcomeFuture: Future<Output = Responded> + Send + 'static,
{
    async fn handle(&self, request: Request) -> Responded {
        (self.extract)(self.responder.clone(), request).await
    }
}

pub fn responder_handler<Responder, Extract, OutcomeFuture>(
    responder: Arc<Responder>,
    extract: Extract,
) -> Arc<dyn Handler>
where
    Responder: Send + Sync + 'static,
    Extract: Fn(Arc<Responder>, Request) -> OutcomeFuture + Send + Sync + 'static,
    OutcomeFuture: Future<Output = Responded> + Send + 'static,
{
    Arc::new(FnHandler { extract, responder })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use http_body_util::BodyExt;

    use super::responder_handler;
    use crate::method::Method;
    use crate::request::Request;
    use crate::responded::Responded;
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
                Responded::from(
                    responder
                        .respond(
                            request
                                .path_param("id")
                                .expect("the test request carries the id parameter")
                                .to_string(),
                        )
                        .await,
                )
            },
        );
        let mut request = Request::new(Method::Get, "/echo/7".to_string());
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "7".to_string());
        request.set_path_params(path_params);

        let response = handler.handle(request).await.into_http();

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
