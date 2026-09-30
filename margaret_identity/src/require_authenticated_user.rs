use margaret_http::requirement::Requirement;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

#[must_use]
pub fn require_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Requirement<User> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => {
            Requirement::Unmet(ResponseContinuation::from(Response::unauthorized()))
        }
        AuthenticatedUserOutcome::Authenticated(user) => Requirement::Met(user),
        AuthenticatedUserOutcome::Interrupted(continuation) => Requirement::Unmet(continuation),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::requirement::Requirement;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http::url_segment::UrlSegment;

    use super::require_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn rejection_status(outcome: AuthenticatedUserOutcome<&str>) -> Option<u16> {
        match require_authenticated_user(outcome) {
            Requirement::Unmet(ResponseContinuation::Done(response)) => Some(response.status()),
            Requirement::Met(_)
            | Requirement::Unmet(
                ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_),
            ) => None,
        }
    }

    fn sign_in_redirect() -> ResponseContinuation {
        ResponseContinuation::from(
            ForwardableRoute::new(
                Arc::from("http://localhost"),
                vec![UrlSegment::Literal("/sign-in")],
            )
            .see_other(),
        )
    }

    #[test]
    fn hands_over_the_authenticated_user() {
        assert!(matches!(
            require_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")),
            Requirement::Met("milo")
        ));
    }

    #[test]
    fn rejects_an_anonymous_visitor_as_unauthorized() {
        assert_eq!(
            rejection_status(AuthenticatedUserOutcome::Anonymous),
            Some(401)
        );
    }

    #[test]
    fn propagates_a_terminal_interruption_instead_of_rejecting() {
        assert_eq!(
            rejection_status(AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(Response::text(410, "the session expired"))
            )),
            Some(410)
        );
    }

    #[test]
    fn keeps_a_redirecting_interruption_intact() {
        assert_eq!(
            rejection_status(AuthenticatedUserOutcome::Interrupted(sign_in_redirect())),
            None
        );
    }
}
