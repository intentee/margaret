use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::identity_session::access_token_claims::AccessTokenClaims;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::models::access_token_holder::AccessTokenHolder;

#[singleton]
#[infers_authenticated_user(user_model = AccessTokenHolder)]
pub struct AccessTokenHolderProvider;

impl AccessTokenHolderProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_access_token_holder(
        &self,
        #[bearer_token(issuer = auth)] token: Option<VerifiedJwt<AccessTokenClaims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<AccessTokenHolder>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(AccessTokenHolder {
                subject: verified.claims.sub,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
