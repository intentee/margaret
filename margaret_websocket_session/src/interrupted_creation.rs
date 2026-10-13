use margaret_http::response_continuation::ResponseContinuation;

use crate::web_socket_session_creation_error::WebSocketSessionCreationError;
use crate::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;

/// # Errors
///
/// Never fails; the result answers the handshake with the continuation in place of a session.
pub fn interrupted_creation<TSession>(
    continuation: ResponseContinuation,
) -> Result<WebSocketSessionCreationOutcome<TSession>, WebSocketSessionCreationError> {
    Ok(WebSocketSessionCreationOutcome::Interrupted(continuation))
}

#[cfg(test)]
mod tests {
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;

    use super::interrupted_creation;
    use crate::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;

    #[test]
    fn answers_the_handshake_with_the_continuation_instead_of_a_session() {
        assert!(matches!(
            interrupted_creation::<()>(ResponseContinuation::from(Response::forbidden())),
            Ok(WebSocketSessionCreationOutcome::Interrupted(ResponseContinuation::Done(response)))
                if response.status() == 403
        ));
    }
}
