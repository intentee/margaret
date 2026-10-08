use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::identity_session::access_token_claims::AccessTokenClaims;
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use super::caller::Caller;

#[singleton]
#[infers_authenticated_user(user_model = Caller)]
pub struct CallerProvider;

impl CallerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_caller(
        &self,
        #[bearer_token(issuer = upstream)] token: Option<
            VerifiedJwt<AccessTokenClaims, AccessTokenProfile>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Caller>> {
        Ok(match token {
            Some(_) => AuthenticatedUserOutcome::Authenticated(Caller),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
