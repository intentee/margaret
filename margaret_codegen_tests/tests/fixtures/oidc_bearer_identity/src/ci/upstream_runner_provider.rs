use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::ci::upstream_runner::UpstreamRunner;
use crate::upstream::claims::Claims;

#[singleton]
#[infers_authenticated_user(user_model = UpstreamRunner)]
pub struct UpstreamRunnerProvider;

impl UpstreamRunnerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer(
        &self,
        #[bearer_token(issuer = upstream)] token: Option<VerifiedJwt<Claims, IdTokenProfile>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<UpstreamRunner>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(UpstreamRunner {
                repository: verified.claims.repository,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
