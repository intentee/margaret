use margaret_http::requirement::Requirement;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

#[must_use]
pub fn optional_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Requirement<Option<User>> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Requirement::Met(None),
        AuthenticatedUserOutcome::Authenticated(user) => Requirement::Met(Some(user)),
        AuthenticatedUserOutcome::Interrupted(response) => Requirement::Unmet(response),
    }
}

#[cfg(test)]
mod tests {
    use http::header::LOCATION;

    use margaret_http::redirect::Redirect;
    use margaret_http::requirement::Requirement;
    use margaret_http::response::Response;

    use super::optional_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

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
        assert!(matches!(
            optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
                Response::text(410, "the session expired")
            )),
            Requirement::Unmet(response) if response.status() == 410
        ));
    }

    #[test]
    fn keeps_a_sign_in_redirect_intact_instead_of_admitting_an_anonymous_visitor() {
        assert!(matches!(
            optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
                Redirect::see_other("http://localhost/sign-in".to_string()).into_response()
            )),
            Requirement::Unmet(response)
                if response.status() == 303
                    && response.header_value(&LOCATION) == Some("http://localhost/sign-in")
        ));
    }
}
