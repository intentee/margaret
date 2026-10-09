use cookie::Cookie;
use reqwest::Response;
use reqwest::header::SET_COOKIE;

/// # Panics
///
/// Panics when a cookie the response sets is not a cookie.
#[must_use]
pub fn response_cookies(response: &Response) -> String {
    response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .map(|header| {
            Cookie::parse(header.to_str().expect("the cookie is text"))
                .expect("the response sets a cookie")
                .stripped()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("; ")
}
