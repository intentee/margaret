use async_trait::async_trait;

use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: &Request, cookie_jar: &CookieJar) -> ResponseContinuation;
}
