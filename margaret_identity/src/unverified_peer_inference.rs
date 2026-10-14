use margaret_http::cookie_changes::CookieChanges;
use margaret_http::response::Response;

use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
use crate::authenticated_user_presence::AuthenticatedUserPresence;
use crate::session_user_inference::SessionUserInference;

pub trait UnverifiedPeerInference {
    fn unverified_peer(refusal: Response) -> Self;
}

impl<TUser> UnverifiedPeerInference for AuthenticatedUserOutcome<TUser> {
    fn unverified_peer(refusal: Response) -> Self {
        Self::Interrupted(refusal)
    }
}

impl<TUser> UnverifiedPeerInference for AuthenticatedUserPresence<TUser> {
    fn unverified_peer(_refusal: Response) -> Self {
        Self::Anonymous
    }
}

impl<TInference: UnverifiedPeerInference> UnverifiedPeerInference for anyhow::Result<TInference> {
    fn unverified_peer(refusal: Response) -> Self {
        Ok(TInference::unverified_peer(refusal))
    }
}

impl<TInference: UnverifiedPeerInference> UnverifiedPeerInference
    for SessionUserInference<TInference>
{
    fn unverified_peer(refusal: Response) -> Self {
        Self {
            cookie_changes: CookieChanges {
                cookies: Vec::new(),
            },
            outcome: TInference::unverified_peer(refusal),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use margaret_http::response::Response;

    use super::UnverifiedPeerInference;
    use crate::authenticated_user_outcome::AuthenticatedUserOutcome;
    use crate::authenticated_user_presence::AuthenticatedUserPresence;
    use crate::session_user_inference::SessionUserInference;

    #[test]
    fn interrupts_an_outcome_with_the_refusal_of_the_unverified_peer() {
        assert!(matches!(
            AuthenticatedUserOutcome::<()>::unverified_peer(Response::forbidden()),
            AuthenticatedUserOutcome::Interrupted(response)
                if response.status() == 403
        ));
    }

    #[test]
    fn leaves_a_presence_anonymous_since_an_unverified_peer_names_no_user() {
        assert_eq!(
            mem::discriminant(&AuthenticatedUserPresence::<()>::unverified_peer(
                Response::forbidden()
            )),
            mem::discriminant(&AuthenticatedUserPresence::Anonymous)
        );
    }

    #[test]
    fn concludes_a_fallible_inference_successfully_since_an_unverified_peer_is_no_failure() {
        assert_eq!(
            mem::discriminant(
                &anyhow::Result::<AuthenticatedUserPresence<()>>::unverified_peer(
                    Response::forbidden()
                )
                .expect("an unverified peer is no failure")
            ),
            mem::discriminant(&AuthenticatedUserPresence::Anonymous)
        );
    }

    #[test]
    fn concludes_a_session_inference_before_any_session_cookie_changes() {
        assert!(matches!(
            SessionUserInference::<AuthenticatedUserOutcome<()>>::unverified_peer(Response::forbidden()),
            SessionUserInference {
                cookie_changes,
                outcome: AuthenticatedUserOutcome::Interrupted(response),
            } if cookie_changes.cookies.is_empty() && response.status() == 403
        ));
    }
}
