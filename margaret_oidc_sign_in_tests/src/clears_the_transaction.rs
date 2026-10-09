use cookie::Cookie;
use http::header::SET_COOKIE;

use margaret_http::response::Response;

/// # Panics
///
/// Panics when a set-cookie header of the response does not parse.
#[must_use]
pub fn clears_the_transaction(response: &Response) -> bool {
    response
        .headers()
        .iter()
        .filter(|header| header.name == SET_COOKIE.as_str())
        .map(|header| Cookie::parse(header.value.clone()).expect("the set-cookie header parses"))
        .any(|cookie| {
            cookie.name().starts_with("__Host-margaret-sign-in-")
                && cookie.max_age() == Some(cookie::time::Duration::ZERO)
        })
}
