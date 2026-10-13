use http::header::WWW_AUTHENTICATE;

use crate::response::Response;

#[must_use]
pub fn basic_challenged(response: Response) -> Response {
    response.header(WWW_AUTHENTICATE.as_str(), "Basic")
}
