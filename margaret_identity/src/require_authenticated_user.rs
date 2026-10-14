use margaret_http::requirement::Requirement;
use margaret_http::response::Response;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

#[must_use]
pub fn require_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Requirement<User> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Requirement::Unmet(Response::unauthorized()),
        AuthenticatedUserOutcome::Authenticated(user) => Requirement::Met(user),
        AuthenticatedUserOutcome::Interrupted(response) => Requirement::Unmet(response),
    }
}

#[cfg(test)]
mod tests {
    use http::header::LOCATION;

    use margaret_http::redirect::Redirect;
    use margaret_http::requirement::Requirement;
    use margaret_http::response::Response;

    use super::require_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    #[test]
    fn meets_the_requirement_with_the_authenticated_user_so_handlers_receive_it() {
        assert!(matches!(
            require_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")),
            Requirement::Met("milo")
        ));
    }

    #[test]
    fn rejects_an_anonymous_visitor_as_unauthorized_so_protected_pages_stay_private() {
        assert!(matches!(
            require_authenticated_user::<&str>(AuthenticatedUserOutcome::Anonymous),
            Requirement::Unmet(response) if response.status() == 401
        ));
    }

    #[test]
    fn propagates_a_terminal_interruption_instead_of_rejecting() {
        assert!(matches!(
            require_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
                Response::text(410, "the session expired")
            )),
            Requirement::Unmet(response) if response.status() == 410
        ));
    }

    #[test]
    fn keeps_a_sign_in_redirect_intact_instead_of_replacing_it_with_a_401() {
        assert!(matches!(
            require_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
                Redirect::see_other("http://localhost/sign-in".to_string()).into_response()
            )),
            Requirement::Unmet(response)
                if response.status() == 303
                    && response.header_value(&LOCATION) == Some("http://localhost/sign-in")
        ));
    }
}
