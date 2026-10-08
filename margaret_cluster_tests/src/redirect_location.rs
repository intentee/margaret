use reqwest::Response;
use reqwest::header::LOCATION;
use url::Url;

/// # Panics
///
/// Panics when the response does not redirect to a URL.
#[must_use]
pub fn redirect_location(response: &Response) -> Url {
    assert!(response.status().is_redirection());

    Url::parse(
        response
            .headers()
            .get(LOCATION)
            .expect("the response redirects")
            .to_str()
            .expect("the redirect is text"),
    )
    .expect("the redirect is a URL")
}
