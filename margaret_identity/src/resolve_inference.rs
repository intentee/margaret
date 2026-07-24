use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::responds_to_inference_failure::RespondsToInferenceFailure;

#[must_use]
pub fn resolve_inference<User, Error>(
    inferred: Result<AuthenticatedUserOutcome<User>, Error>,
) -> AuthenticatedUserOutcome<User>
where
    Error: RespondsToInferenceFailure,
{
    match inferred {
        Ok(outcome) => outcome,
        Err(error) => AuthenticatedUserOutcome::Interrupted(error.into_response_continuation()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http::url_segment::UrlSegment;

    use super::resolve_inference;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
    use crate::require_authenticated_user::require_authenticated_user;
    use crate::responds_to_inference_failure::RespondsToInferenceFailure;

    struct UnreachableSessionStore;

    impl RespondsToInferenceFailure for UnreachableSessionStore {
        fn into_response_continuation(self) -> ResponseContinuation {
            ResponseContinuation::from(Response::text(503, "the session store is unreachable"))
        }
    }

    struct ExpiredSession;

    impl RespondsToInferenceFailure for ExpiredSession {
        fn into_response_continuation(self) -> ResponseContinuation {
            ResponseContinuation::from(
                ForwardableRoute::new(
                    Arc::from("http://localhost"),
                    vec![UrlSegment::Literal("/sign-in")],
                )
                .see_other(),
            )
        }
    }

    fn interruption_status<Failure: RespondsToInferenceFailure>(failure: Failure) -> Option<u16> {
        let rejection =
            require_authenticated_user(resolve_inference::<&str, Failure>(Err(failure)))
                .expect_err("a failed inference carries no user");

        match rejection {
            ResponseContinuation::Done(response) => Some(response.status()),
            ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_) => None,
        }
    }

    #[test]
    fn hands_over_a_successful_inference_unchanged() {
        let resolved = resolve_inference(Ok::<_, UnreachableSessionStore>(
            AuthenticatedUserOutcome::Authenticated("milo"),
        ));

        assert_eq!(require_authenticated_user(resolved).ok(), Some("milo"));
    }

    #[test]
    fn interrupts_the_request_with_the_response_the_failure_asks_for() {
        assert_eq!(interruption_status(UnreachableSessionStore), Some(503));
    }

    #[test]
    fn lets_a_failure_redirect_instead_of_responding() {
        assert_eq!(interruption_status(ExpiredSession), None);
    }
}
