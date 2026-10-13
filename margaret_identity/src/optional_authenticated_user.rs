use margaret_http::requirement::Requirement;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

#[must_use]
pub fn optional_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Requirement<Option<User>> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Requirement::Met(None),
        AuthenticatedUserOutcome::Authenticated(user) => Requirement::Met(Some(user)),
        AuthenticatedUserOutcome::Interrupted(continuation) => Requirement::Unmet(continuation),
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::requirement::Requirement;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;

    use super::optional_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn interruption_status(continuation: ResponseContinuation) -> Option<u16> {
        match optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
            continuation,
        )) {
            Requirement::Unmet(ResponseContinuation::Done(response)) => Some(response.status()),
            Requirement::Met(_)
            | Requirement::Unmet(
                ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_),
            ) => None,
        }
    }

    #[test]
    fn meets_the_optional_requirement_with_the_user_so_handlers_see_who_signed_in() {
        assert!(matches!(
            optional_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")),
            Requirement::Met(Some("milo"))
        ));
    }

    #[test]
    fn admits_an_anonymous_visitor_without_a_user() {
        assert!(matches!(
            optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Anonymous),
            Requirement::Met(user) if user.is_none()
        ));
    }

    #[test]
    fn propagates_a_terminal_interruption_response_instead_of_admitting_an_anonymous_visitor() {
        assert_eq!(
            interruption_status(ResponseContinuation::from(Response::text(
                410,
                "the session expired"
            ))),
            Some(410)
        );
    }

    #[test]
    fn keeps_a_sign_in_redirect_intact_instead_of_admitting_an_anonymous_visitor() {
        assert_eq!(
            interruption_status(ResponseContinuation::from(
                ForwardableRoute::new("http://localhost/sign-in".to_string()).see_other(),
            )),
            None
        );
    }
}
