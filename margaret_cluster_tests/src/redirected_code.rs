use reqwest::Response;
use reqwest::StatusCode;

use crate::redirect_location::redirect_location;

/// # Panics
///
/// Panics when the response does not redirect with an authorization code.
#[must_use]
pub fn redirected_code(response: &Response) -> String {
    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    redirect_location(response)
        .query_pairs()
        .find(|(parameter, _value)| parameter == "code")
        .expect("the redirect carries a code")
        .1
        .into_owned()
}
