use async_trait::async_trait;

use margaret_cookie_jar::cookie_jar::CookieJar;

use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait HttpMiddleware: Send + Sync {
    async fn process<'jar>(
        &self,
        request: &Request,
        cookie_jar: &'jar CookieJar,
        next: Next<'jar>,
    ) -> ResponseContinuation;
}
