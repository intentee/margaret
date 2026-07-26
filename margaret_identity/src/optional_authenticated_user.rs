use margaret_http::redirect::Redirect;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::respond_with_identity_error::respond_with_identity_error;

pub fn optional_authenticated_user<User>(
    outcome: anyhow::Result<AuthenticatedUserOutcome<User>>,
) -> Result<Option<User>, ResponseContinuation> {
    match outcome {
        Ok(AuthenticatedUserOutcome::Anonymous) => Ok(None),
        Ok(AuthenticatedUserOutcome::Authenticated(user)) => Ok(Some(user)),
        Ok(AuthenticatedUserOutcome::LoginPageRedirect { url }) => {
            Err(ResponseContinuation::from(Redirect::see_other(url)))
        }
        Err(error) => Err(respond_with_identity_error(error)),
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::response_continuation::ResponseContinuation;

    use super::optional_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn rejection_status(outcome: anyhow::Result<AuthenticatedUserOutcome<&str>>) -> Option<u16> {
        let rejection =
            optional_authenticated_user(outcome).expect_err("the outcome carries no user");

        match rejection {
            ResponseContinuation::Done(response) => Some(response.status()),
            ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_) => None,
        }
    }

    #[test]
    fn hands_over_the_authenticated_user() {
        assert_eq!(
            optional_authenticated_user(Ok(AuthenticatedUserOutcome::Authenticated("milo"))).ok(),
            Some(Some("milo"))
        );
    }

    #[test]
    fn admits_an_anonymous_visitor_without_a_user() {
        assert_eq!(
            optional_authenticated_user::<&str>(Ok(AuthenticatedUserOutcome::Anonymous)).ok(),
            Some(None)
        );
    }

    #[test]
    fn redirects_a_login_page_outcome() {
        assert_eq!(
            rejection_status(Ok(AuthenticatedUserOutcome::LoginPageRedirect {
                url: "http://localhost/sign-in".to_string(),
            })),
            None
        );
    }

    #[test]
    fn maps_a_provider_error_to_an_internal_server_error() {
        assert_eq!(
            rejection_status(Err(anyhow::anyhow!("the session store is unavailable"))),
            Some(500)
        );
    }
}
