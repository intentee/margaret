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
    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::requirement::Requirement;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;

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
            ForwardableRoute::new("http://localhost/sign-in".to_string()).see_other(),
        )
    }

    #[test]
    fn meets_the_requirement_with_the_authenticated_user_so_handlers_receive_it() {
        assert!(matches!(
            require_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")),
            Requirement::Met("milo")
        ));
    }

    #[test]
    fn rejects_an_anonymous_visitor_as_unauthorized_so_protected_pages_stay_private() {
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
    fn keeps_a_sign_in_redirect_intact_instead_of_replacing_it_with_a_401() {
        assert_eq!(
            rejection_status(AuthenticatedUserOutcome::Interrupted(sign_in_redirect())),
            None
        );
    }
}
