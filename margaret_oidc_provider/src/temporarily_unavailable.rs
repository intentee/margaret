use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;

use crate::oauth_error::oauth_error;

pub(crate) fn temporarily_unavailable(description: &'static str) -> Response {
    oauth_error(
        503,
        BasicErrorResponseType::Extension("temporarily_unavailable".to_string()),
        description,
    )
}
