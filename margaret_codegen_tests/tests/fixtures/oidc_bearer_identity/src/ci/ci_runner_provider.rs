use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::ci::ci_runner::CiRunner;
use crate::partner;

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
        #[bearer_token(issuer = partner)] token: Option<VerifiedJwt<partner::claims::Claims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<CiRunner>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(CiRunner {
                repository: verified.claims.repository,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
