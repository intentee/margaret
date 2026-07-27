use margaret_http::response_continuation::ResponseContinuation;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

pub fn optional_authenticated_user<User>(
    outcome: AuthenticatedUserOutcome<User>,
) -> Result<Option<User>, ResponseContinuation> {
    match outcome {
        AuthenticatedUserOutcome::Anonymous => Ok(None),
        AuthenticatedUserOutcome::Authenticated(user) => Ok(Some(user)),
        AuthenticatedUserOutcome::Interrupted(continuation) => Err(continuation),
    }
}

#[cfg(test)]
mod tests {
    use margaret_http::forwardable_route::ForwardableRoute;
    use margaret_http::response::Response;
    use margaret_http::response_continuation::ResponseContinuation;
    use margaret_http::route_origin::RouteOrigin;
    use margaret_http::url_segment::UrlSegment;

    use super::optional_authenticated_user;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;

    fn interruption_status(continuation: ResponseContinuation) -> Option<u16> {
        let rejection = optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Interrupted(
            continuation,
        ))
        .expect_err("the interruption is propagated");

        match rejection {
            ResponseContinuation::Done(response) => Some(response.status()),
            ResponseContinuation::Forward(_) | ResponseContinuation::Redirect(_) => None,
        }
    }

    #[test]
    fn hands_over_the_authenticated_user() {
        assert_eq!(
            optional_authenticated_user(AuthenticatedUserOutcome::Authenticated("milo")).ok(),
            Some(Some("milo"))
        );
    }

    #[test]
    fn admits_an_anonymous_visitor_without_a_user() {
        assert_eq!(
            optional_authenticated_user::<&str>(AuthenticatedUserOutcome::Anonymous).ok(),
            Some(None)
        );
    }

    #[test]
    fn propagates_a_terminal_interruption() {
        assert_eq!(
            interruption_status(ResponseContinuation::from(Response::text(
                410,
                "the session expired"
            ))),
            Some(410)
        );
    }

    #[test]
    fn keeps_a_redirecting_interruption_intact() {
        assert_eq!(
            interruption_status(ResponseContinuation::from(
                ForwardableRoute::new(
                    RouteOrigin::parse("https://example.test").expect("a valid origin"),
                    vec![UrlSegment::Literal("/sign-in")],
                )
                .see_other(),
            )),
            None
        );
    }
}
