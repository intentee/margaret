use margaret_http::requirement::Requirement;
use margaret_http::response::Response;

use crate::authenticated_user_presence::AuthenticatedUserPresence;

#[must_use]
pub fn require_present_user<User>(presence: AuthenticatedUserPresence<User>) -> Requirement<User> {
    match presence {
        AuthenticatedUserPresence::Anonymous => Requirement::Unmet(Response::unauthorized()),
        AuthenticatedUserPresence::Authenticated(user) => Requirement::Met(user),
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::requirement::Requirement;

    use super::require_present_user;
    use crate::authenticated_user_presence::AuthenticatedUserPresence;

    #[test]
    fn meets_the_requirement_with_the_present_user() {
        assert!(matches!(
            require_present_user(AuthenticatedUserPresence::Authenticated("milo")),
            Requirement::Met("milo")
        ));
    }

    #[test]
    fn answers_an_anonymous_visitor_as_unauthorized() {
        assert!(matches!(
            require_present_user::<&str>(AuthenticatedUserPresence::Anonymous),
            Requirement::Unmet(response) if response.status() == 401
        ));
    }
}
