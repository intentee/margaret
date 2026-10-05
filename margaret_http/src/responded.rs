use crate::handler_error::HandlerError;
use crate::response_continuation::ResponseContinuation;

/// # Errors
///
/// Returns `HandlerError::Consumer` carrying the error the responder reported.
pub fn responded<TResponse: Into<ResponseContinuation>>(
    outcome: anyhow::Result<TResponse>,
) -> Result<ResponseContinuation, HandlerError> {
    outcome.map(Into::into).map_err(HandlerError::consumer)
}
