use std::sync::Arc;

use margaret_cookie_jar::cookie_jar::CookieJar;
use margaret_http::next::Next;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_macros::constructor;
use margaret_macros::handles_middleware_attribute;
use margaret_macros::process;
use margaret_macros::singleton;

use crate::log_sink::LogSink;

#[singleton]
#[handles_middleware_attribute(attribute = cookie_audited)]
pub struct CookieAudit {
    sinks: Vec<Arc<dyn LogSink>>,
}

impl CookieAudit {
    #[constructor]
    pub fn create(sinks: Vec<Arc<dyn LogSink>>) -> Self {
        Self { sinks }
    }

    #[process]
    pub async fn process(
        &self,
        request: &Request,
        cookies: &CookieJar,
        next: Next,
    ) -> ResponseContinuation {
        let line = format!(
            "{} carried {} cookies",
            request.inputs.server.path(),
            cookies.iter().count()
        );

        for sink in &self.sinks {
            sink.write(&line);
        }

        next.run(request).await
    }
}
