use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

pub fn require_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Result<User, ResponseContinuation> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => {
            Err(ResponseContinuation::from(Response::unauthorized()))
        }
        AuthenticatedUserOutcome::Authenticated(user) => Ok(user),
        AuthenticatedUserOutcome::Interrupted(continuation) => Err(continuation),
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http::route_origin::RouteOrigin;
    use margaret_http::url_segment::UrlSegment;

    use super::require_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn rejection_status(outcome: AuthenticatedUserOutcome<&str>) -> Option<u16> {
        let rejection =
            require_authenticated_user(outcome).expect_err("the outcome carries no user");

        match rejection {
            ResponseContinuation::Done(response) => Some(response.status()),
            ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_) => None,
        }
    }

    fn sign_in_redirect() -> ResponseContinuation {
        ResponseContinuation::from(
            ForwardableRoute::new(
                RouteOrigin::parse("https://example.test").expect("a valid origin"),
                vec![UrlSegment::Literal("/sign-in")],
            )
            .see_other(),
        )
    }

    #[test]
    fn hands_over_the_authenticated_user() {
        assert_eq!(
            require_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")).ok(),
            Some("milo")
        );
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
