use crate::authenticated_user_presence::AuthenticatedUserPresence;

#[must_use]
pub fn optional_present_user<User>(presence: AuthenticatedUserPresence<User>) -> Option<User> {
    match presence {
        AuthenticatedUserPresence::Anonymous => None,
        AuthenticatedUserPresence::Authenticated(user) => Some(user),
    }
}

#[cfg(test)]
mod tests {
    use super::optional_present_user;
    use crate::authenticated_user_presence::AuthenticatedUserPresence;

    #[test]
    fn hands_over_the_present_user() {
        assert_eq!(
            optional_present_user(AuthenticatedUserPresence::Authenticated("milo")),
            Some("milo")
        );
    }

    #[test]
    fn admits_an_anonymous_visitor_without_a_user() {
        assert_eq!(
            optional_present_user::<&str>(AuthenticatedUserPresence::Anonymous),
            None
        );
    }
}
