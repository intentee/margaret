use cookie::Cookie;
use cookie::CookieBuilder;
use cookie::SameSite;

#[must_use]
pub fn host_cookie(name: String, value: String) -> CookieBuilder<'static> {
    Cookie::build((name, value))
        .http_only(true)
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
}
