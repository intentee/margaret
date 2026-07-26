use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::identity_error::IdentityError;

#[must_use]
pub fn respond_with_identity_error(error: anyhow::Error) -> ResponseContinuation {
    eprintln!("{}", IdentityError::UserError(error));

    ResponseContinuation::Done(Response::internal_server_error())
}
