use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_client::oidc_token_verification::OidcTokenVerification;

use crate::auth::github_actions_claims::GithubActionsClaims;
use crate::models::ci_runner::CiRunner;

#[singleton]
#[infers_authenticated_user(user_model = CiRunner)]
pub struct CiRunnerProvider {
    repository_owner_id: String,
}

impl CiRunnerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "github-actions-repository-owner-id")]
        repository_owner_id: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            repository_owner_id,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_ci_runner(
        &self,
        #[oidc_token(issuer = github_actions)] verification: OidcTokenVerification<
            GithubActionsClaims,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<CiRunner>> {
        Ok(match verification {
            OidcTokenVerification::Absent | OidcTokenVerification::NotBearer => {
                AuthenticatedUserOutcome::Anonymous
            }
            OidcTokenVerification::Rejected(rejection) => AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(rejection.challenge().response()),
            ),
            OidcTokenVerification::Unavailable => {
                AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::text(
                    503,
                    "the GitHub Actions signing keys are not available yet",
                )))
            }
            OidcTokenVerification::Verified(verified) => {
                let GithubActionsClaims {
                    repository,
                    repository_id,
                    repository_owner_id,
                } = verified.claims;

                if repository_owner_id == self.repository_owner_id {
                    AuthenticatedUserOutcome::Authenticated(CiRunner {
                        repository,
                        repository_id,
                    })
                } else {
                    AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(
                        Response::forbidden(),
                    ))
                }
            }
        })
    }
}
