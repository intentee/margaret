use margaret_handler_error::handler_error::HandlerError;

use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

/// # Errors
///
/// Never fails; the refusal answers the request with its response.
pub fn refusal<TTarget>(response: Response) -> Result<ResponseContinuation<TTarget>, HandlerError> {
    Ok(ResponseContinuation::Done(response))
}
