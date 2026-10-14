use margaret_handler_error::handler_error::HandlerError;

use crate::response_continuation::ResponseContinuation;

/// # Errors
///
/// Returns `HandlerError::Consumer` carrying the error the responder reported.
pub fn responded<TTarget, TResponse: Into<ResponseContinuation<TTarget>>>(
    outcome: anyhow::Result<TResponse>,
) -> Result<ResponseContinuation<TTarget>, HandlerError> {
    outcome.map(Into::into).map_err(HandlerError::consumer)
}
