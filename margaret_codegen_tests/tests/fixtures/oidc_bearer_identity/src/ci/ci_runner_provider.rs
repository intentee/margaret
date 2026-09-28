use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_client::oidc_token_verification::OidcTokenVerification;

use crate::ci::ci_runner::CiRunner;
use crate::partner;
use crate::upstream;

#[singleton]
#[infers_authenticated_user(user_model = CiRunner)]
pub struct CiRunnerProvider;

impl CiRunnerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer(
        &self,
        #[oidc_token(issuer = partner)] partner_token: OidcTokenVerification<
            partner::claims::Claims,
        >,
        #[oidc_token(issuer = upstream)] upstream_token: OidcTokenVerification<
            upstream::claims::Claims,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<CiRunner>> {
        if let OidcTokenVerification::Verified(verified) = partner_token {
            return Ok(AuthenticatedUserOutcome::Authenticated(CiRunner {
                repository: verified.claims.repository,
            }));
        }

        Ok(match upstream_token {
            OidcTokenVerification::Verified(verified) => {
                AuthenticatedUserOutcome::Authenticated(CiRunner {
                    repository: verified.claims.repository,
                })
            }
            OidcTokenVerification::Rejected(rejection) => AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(rejection.challenge().response()),
            ),
            OidcTokenVerification::Absent
            | OidcTokenVerification::NotBearer
            | OidcTokenVerification::Unavailable => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
