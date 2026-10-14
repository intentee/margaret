use margaret_http::response::Response;

use crate::web_socket_session_creation_error::WebSocketSessionCreationError;
use crate::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;

/// # Errors
///
/// Never fails; the result answers the handshake with the response in place of a session.
pub fn interrupted_creation<TSession>(
    response: Response,
) -> Result<WebSocketSessionCreationOutcome<TSession>, WebSocketSessionCreationError> {
    Ok(WebSocketSessionCreationOutcome::Interrupted(response))
}

#[cfg(test)]
mod tests {
    use margaret_http::response::Response;

    use super::interrupted_creation;
    use crate::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;

    #[test]
    fn answers_the_handshake_with_the_response_instead_of_a_session() {
        assert!(matches!(
            interrupted_creation::<()>(Response::forbidden()),
            Ok(WebSocketSessionCreationOutcome::Interrupted(response))
                if response.status() == 403
        ));
    }
}
