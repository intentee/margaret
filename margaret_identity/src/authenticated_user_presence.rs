use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

pub enum AuthenticatedUserPresence<User> {
    Anonymous,
    Authenticated(User),
}

impl<User> From<AuthenticatedUserPresence<User>> for AuthenticatedUserOutcome<User> {
    fn from(presence: AuthenticatedUserPresence<User>) -> Self {
        match presence {
            AuthenticatedUserPresence::Anonymous => Self::Anonymous,
            AuthenticatedUserPresence::Authenticated(user) => Self::Authenticated(user),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::AuthenticatedUserPresence;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    #[test]
    fn keeps_an_anonymous_visitor_anonymous_as_an_outcome() {
        assert_eq!(
            mem::discriminant(&AuthenticatedUserOutcome::from(
                AuthenticatedUserPresence::<&str>::Anonymous
            )),
            mem::discriminant(&AuthenticatedUserOutcome::Anonymous)
        );
    }

    #[test]
    fn keeps_an_authenticated_user_as_an_outcome() {
        assert!(matches!(
            AuthenticatedUserOutcome::from(AuthenticatedUserPresence::Authenticated("milo")),
            AuthenticatedUserOutcome::Authenticated("milo")
        ));
    }
}
