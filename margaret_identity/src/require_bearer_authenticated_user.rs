use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::require_authenticated_user::require_authenticated_user;

/// # Errors
///
/// Returns `ResponseContinuation` that challenges an anonymous visitor for bearer credentials, or the interruption of the provider.
pub fn require_bearer_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Result<User, ResponseContinuation> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Err(ResponseContinuation::from(
            BearerChallenge::MissingCredentials.response(),
        )),
        AuthenticatedUserOutcome::Authenticated(_) | AuthenticatedUserOutcome::Interrupted(_) => {
            require_authenticated_user(outcome)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::require_bearer_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    #[test]
    fn hands_over_the_authenticated_user() {
        assert_eq!(
            require_bearer_authenticated_user(AuthenticatedUserOutcome::Authenticated("runner"))
                .ok(),
            Some("runner")
        );
    }
}
