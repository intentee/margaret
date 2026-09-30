use oauth2::StandardErrorResponse;
use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;

use crate::no_store::no_store;

pub(crate) fn oauth_error(
    status: u16,
    error: BasicErrorResponseType,
    description: &str,
) -> Response {
    no_store(Response::json(
        status,
        &StandardErrorResponse::new(error, Some(description.to_string()), None),
    ))
}
