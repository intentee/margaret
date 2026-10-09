use cookie::Cookie;
use reqwest::Response;
use reqwest::header::SET_COOKIE;

/// # Panics
///
/// Panics when the response sets no cookie of the name or a cookie that does not parse.
#[must_use]
pub fn response_cookie(response: &Response, name: &str) -> String {
    response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .map(|header| {
            Cookie::parse(header.to_str().expect("the cookie is text"))
                .expect("the response sets a cookie")
        })
        .find(|cookie| cookie.name() == name)
        .expect("the response sets the cookie")
        .stripped()
        .to_string()
}
