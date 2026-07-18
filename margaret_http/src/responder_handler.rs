use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::handler::Handler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct FnHandler<Responder, Extract> {
    extract: Extract,
    responder: Arc<Responder>,
}

#[async_trait]
impl<Responder, Extract> Handler for FnHandler<Responder, Extract>
where
    Responder: Send + Sync + 'static,
    Extract: for<'request> Fn(
            Arc<Responder>,
            &'request Request,
            &'request CookieJar,
        )
            -> Pin<Box<dyn Future<Output = ResponseContinuation> + Send + 'request>>
        + Send
        + Sync
        + 'static,
{
    async fn handle(&self, request: &Request, cookie_jar: &CookieJar) -> ResponseContinuation {
        (self.extract)(self.responder.clone(), request, cookie_jar).await
    }
}

pub fn responder_handler<Responder, Extract>(
    responder: Arc<Responder>,
    extract: Extract,
) -> Arc<dyn Handler>
where
    Responder: Send + Sync + 'static,
    Extract: for<'request> Fn(
            Arc<Responder>,
            &'request Request,
            &'request CookieJar,
        )
            -> Pin<Box<dyn Future<Output = ResponseContinuation> + Send + 'request>>
        + Send
        + Sync
        + 'static,
{
    Arc::new(FnHandler { extract, responder })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;

    use http::HeaderMap;
    use http::Method;
    use http_body_util::BodyExt;

    use margaret_cookie_jar::cookie_jar::CookieJar;

    use super::responder_handler;
    use crate::forward_targets::ForwardTargets;
    use crate::request::Request;
    use crate::respond_recursively::respond_recursively;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

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
            |responder: Arc<Echo>,
             request: &Request,
             _cookie_jar: &CookieJar|
             -> Pin<Box<dyn Future<Output = ResponseContinuation> + Send + '_>> {
                Box::pin(async move {
                    ResponseContinuation::from(
                        responder
                            .respond(
                                request
                                    .path_param("id")
                                    .expect("the test request carries the id parameter")
                                    .to_string(),
                            )
                            .await,
                    )
                })
            },
        );
        let request = Request::new(Method::GET, "/echo/7".to_string())
            .with_path_params(HashMap::from([("id".to_string(), "7".to_string())]));
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));

        let response = respond_recursively(
            &forward_targets,
            request,
            &CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built"),
            handler,
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
