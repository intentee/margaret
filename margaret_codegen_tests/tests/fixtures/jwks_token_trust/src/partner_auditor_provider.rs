use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::audit_claims::AuditClaims;
use crate::partner_auditor::PartnerAuditor;

#[singleton]
#[infers_authenticated_user(user_model = PartnerAuditor)]
pub struct PartnerAuditorProvider;

impl PartnerAuditorProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer(
        &self,
        #[bearer_token(issuer = partner)] token: Option<VerifiedJwt<AuditClaims>>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<PartnerAuditor>> {
        Ok(match token {
            Some(verified) => AuthenticatedUserOutcome::Authenticated(PartnerAuditor {
                scope: verified.claims.scope,
            }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
