use std::sync::Arc;

use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::process;
use margaret_macros::singleton;

use crate::log_sink::LogSink;
use crate::margaret::routes::Routes;

#[singleton]
#[handles_middleware_attribute(attribute = logged)]
pub struct RequestLog {
    sinks: Vec<Arc<dyn LogSink>>,
}

impl RequestLog {
    #[constructor]
    #[must_use]
    pub fn create(sinks: Vec<Arc<dyn LogSink>>) -> Self {
        Self { sinks }
    }

    #[process]
    pub async fn process(
        &self,
        request: &Request,
        next: Next,
        routes: &Routes,
    ) -> ResponseContinuation {
        let line = format!(
            "{} {} (home: {})",
            request.inputs.server.method(),
            request.inputs.server.path(),
            routes.public.get_greeting.url(),
        );

        for sink in &self.sinks {
            sink.write(&line);
        }

        next.run(request).await
    }
}
