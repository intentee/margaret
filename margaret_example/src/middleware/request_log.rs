use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::http_middleware::HttpMiddleware;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::singleton;

use crate::log_sink::LogSink;

#[singleton]
#[handles_middleware_attribute(attribute = logged)]
pub struct RequestLog {
    sinks: Vec<Arc<dyn LogSink>>,
}

impl RequestLog {
    #[constructor]
    pub fn create(sinks: Vec<Arc<dyn LogSink>>) -> Self {
        Self { sinks }
    }
}

#[async_trait]
impl HttpMiddleware for RequestLog {
    async fn process(&self, request: &Request, next: Next) -> ResponseContinuation {
        let line = format!(
            "{:?} {}",
            request.server().method(),
            request.server().path()
        );

        for sink in &self.sinks {
            sink.write(&line);
        }

        next.run(request).await
    }
}
