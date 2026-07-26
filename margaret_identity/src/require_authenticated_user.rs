use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::respond_with_identity_error::respond_with_identity_error;

pub fn require_authenticated_user<User>(
    outcome: anyhow::Result<AuthenticatedUserOutcome<User>>,
) -> Result<User, ResponseContinuation> {
    match outcome {
        Ok(AuthenticatedUserOutcome::Anonymous) => {
            Err(ResponseContinuation::from(Response::unauthorized()))
        }
        Ok(AuthenticatedUserOutcome::Authenticated(user)) => Ok(user),
        Ok(AuthenticatedUserOutcome::LoginPageRedirect(redirect)) => {
            Err(ResponseContinuation::from(redirect))
        }
        Ok(AuthenticatedUserOutcome::Rejected(response)) => {
            Err(ResponseContinuation::from(response))
        }
        Err(error) => Err(respond_with_identity_error(error)),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http::url_segment::UrlSegment;

    use super::require_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn rejection_status(outcome: anyhow::Result<AuthenticatedUserOutcome<&str>>) -> Option<u16> {
        let rejection =
            require_authenticated_user(outcome).expect_err("the outcome carries no user");

        match rejection {
            ResponseContinuation::Done(response) => Some(response.status()),
            ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_) => None,
        }
    }

    #[test]
    fn hands_over_the_authenticated_user() {
        assert_eq!(
            require_authenticated_user(Ok(AuthenticatedUserOutcome::Authenticated("milo"))).ok(),
            Some("milo")
        );
    }

    #[test]
    fn rejects_an_anonymous_visitor_as_unauthorized() {
        assert_eq!(
            rejection_status(Ok(AuthenticatedUserOutcome::Anonymous)),
            Some(401)
        );
    }

    #[test]
    fn redirects_a_login_page_outcome() {
        let sign_in_redirect = ForwardableRoute::new(
            Arc::from("http://localhost"),
            vec![UrlSegment::Literal("/sign-in")],
        )
        .see_other();

        assert_eq!(
            rejection_status(Ok(AuthenticatedUserOutcome::LoginPageRedirect(sign_in_redirect))),
            None
        );
    }

    #[test]
    fn surfaces_a_rejected_extraction_response() {
        assert_eq!(
            rejection_status(Ok(AuthenticatedUserOutcome::Rejected(Response::text(
                400,
                "the credentials are malformed"
            )))),
            Some(400)
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
