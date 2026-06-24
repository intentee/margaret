use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::http_responder::HttpResponder;
use crate::request::Request;
use crate::response::Response;

pub fn responder_handler(responder: Arc<dyn HttpResponder + Send + Sync>) -> Arc<dyn Handler> {
    Arc::new(ResponderHandler { responder })
}

struct ResponderHandler {
    responder: Arc<dyn HttpResponder + Send + Sync>,
}

#[async_trait]
impl Handler for ResponderHandler {
    async fn handle(&self, request: Request) -> Response {
        self.responder.respond(request).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::responder_handler;
    use crate::http_responder::HttpResponder;
    use crate::method::Method;
    use crate::request::Request;
    use crate::response::Response;

    struct Echo;

    #[async_trait]
    impl HttpResponder for Echo {
        async fn respond(&self, _request: Request) -> Response {
            Response::text(200, "echo")
        }
    }

    #[tokio::test]
    async fn calls_the_wrapped_responder() {
        let handler = responder_handler(Arc::new(Echo));
        let response = handler
            .handle(Request::new(Method::Get, "/".to_string()))
            .await
            .into_http();

        assert_eq!(response.status().as_u16(), 200);
    }
}
