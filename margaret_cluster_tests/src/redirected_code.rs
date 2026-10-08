use reqwest::Response;
use reqwest::StatusCode;
use reqwest::header::LOCATION;
use url::Url;

/// # Panics
///
/// Panics when the response does not redirect with an authorization code.
#[must_use]
pub fn redirected_code(response: &Response) -> String {
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    Url::parse(
        response
            .headers()
            .get(LOCATION)
            .expect("the response redirects")
            .to_str()
            .expect("the redirect is text"),
    )
    .expect("the redirect is a URL")
    .query_pairs()
    .find(|(parameter, _value)| parameter == "code")
    .expect("the redirect carries a code")
    .1
    .into_owned()
}
