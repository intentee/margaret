use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::external_claims::ExternalClaims;
use crate::models::external_caller::ExternalCaller;

#[singleton]
#[infers_authenticated_user(user_model = ExternalCaller)]
pub struct ExternalCallerProvider;

impl ExternalCallerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_external_caller(
        &self,
        #[bearer_token(issuer = external)] token: Option<
            VerifiedJwt<ExternalClaims, IdTokenProfile>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<ExternalCaller>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(ExternalCaller {
                subject: verified.claims.sub,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
