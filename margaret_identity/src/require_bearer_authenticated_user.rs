use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::requirement::Requirement;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::require_authenticated_user::require_authenticated_user;

#[must_use]
pub fn require_bearer_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Requirement<User> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Requirement::Unmet(ResponseContinuation::from(
            BearerChallenge::MissingCredentials.response(),
        )),
        AuthenticatedUserOutcome::Authenticated(_) | AuthenticatedUserOutcome::Interrupted(_) => {
            require_authenticated_user(outcome)
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::requirement::Requirement;

    use super::require_bearer_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    #[test]
    fn hands_over_the_authenticated_user() {
        assert!(matches!(
            require_bearer_authenticated_user(AuthenticatedUserOutcome::Authenticated("runner")),
            Requirement::Met("runner")
        ));
    }
}
