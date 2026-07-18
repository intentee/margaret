use std::sync::Arc;

use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::handler::Handler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

pub struct Next<'jar> {
    cookie_jar: &'jar CookieJar,
    inner: Arc<dyn Handler>,
}

impl<'jar> Next<'jar> {
    pub(crate) fn new(inner: Arc<dyn Handler>, cookie_jar: &'jar CookieJar) -> Self {
        Self { cookie_jar, inner }
    }

    pub async fn run(self, request: &Request) -> ResponseContinuation {
        self.inner.handle(request, self.cookie_jar).await
    }
}
