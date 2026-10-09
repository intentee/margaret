use margaret_http::cookie_changes::CookieChanges;
use margaret_http::response_continuation::ResponseContinuation;

use crate::created_web_socket_session::CreatedWebSocketSession;

pub enum WebSocketSessionCreationOutcome<Session> {
    Created(CreatedWebSocketSession<Session>),
    Interrupted(ResponseContinuation),
}

impl<Session> WebSocketSessionCreationOutcome<Session> {
    #[must_use]
    pub fn preceded_by(self, changes: &CookieChanges) -> Self {
        match self {
            Self::Created(CreatedWebSocketSession {
                cookie_changes,
                session,
            }) => Self::Created(CreatedWebSocketSession {
                cookie_changes: changes.followed_by(cookie_changes),
                session,
            }),
            Self::Interrupted(continuation) => Self::Interrupted(changes.precede(continuation)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use cookie::Cookie;

    use margaret_http::cookie_changes::CookieChanges;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;

    use super::WebSocketSessionCreationOutcome;
    use crate::created_web_socket_session::CreatedWebSocketSession;

    fn removed_session() -> CookieChanges {
        CookieChanges {
            cookies: vec![Cookie::new("session", "removed")],
        }
    }

    #[test]
    fn precedes_the_cookie_changes_of_a_created_session() {
        assert!(matches!(
            WebSocketSessionCreationOutcome::Created(CreatedWebSocketSession {
                cookie_changes: CookieChanges {
                    cookies: vec![Cookie::new("session", "started")],
                },
                session: Arc::new(()),
            })
            .preceded_by(&removed_session()),
            WebSocketSessionCreationOutcome::Created(CreatedWebSocketSession { cookie_changes, .. })
                if cookie_changes.cookies.iter().map(Cookie::value).eq(["removed", "started"])
        ));
    }

    #[test]
    fn precedes_the_response_of_an_interruption() {
        assert!(matches!(
            WebSocketSessionCreationOutcome::<()>::Interrupted(ResponseContinuation::from(
                Response::text(401, "")
            ))
            .preceded_by(&removed_session()),
            WebSocketSessionCreationOutcome::Interrupted(ResponseContinuation::Done(response))
                if response.headers().iter().any(|header| header.value == "session=removed")
        ));
    }
}
