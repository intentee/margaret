use std::sync::Arc;

use chrono::Utc;
use url::Url;
use uuid::Uuid;

use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::authenticated_end_user::AuthenticatedEndUser;
use crate::authorization_error::AuthorizationError;
use crate::consent_decision::ConsentDecision;
use crate::consent_outcome::ConsentOutcome;
use crate::provider_error::ProviderError;
use crate::redirection::Redirection;

pub struct ConsentEndpoint {
    grants: Arc<dyn StoresAuthorizationGrants>,
    issuance: TokenIssuance,
}

impl ConsentEndpoint {
    #[must_use]
    pub fn create(grants: Arc<dyn StoresAuthorizationGrants>, issuance: TokenIssuance) -> Self {
        Self { grants, issuance }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::TakePendingAuthorization` when the application cannot take the
    /// pending authorization, and `ProviderError::IssueCode` when it cannot store the approved
    /// code.
    pub async fn decide(
        &self,
        id: Uuid,
        end_user: &AuthenticatedEndUser,
        decision: ConsentDecision,
    ) -> Result<ConsentOutcome, ProviderError> {
        let now = NumericDate::from(Utc::now());
        let PendingAuthorization { grant, state, .. } = match self
            .grants
            .take_pending_authorization(id)
            .await
            .map_err(ProviderError::TakePendingAuthorization)?
        {
            PendingAuthorizationTake::Taken(pending)
                if pending.expires_at > now && pending.grant.subject == end_user.subject =>
            {
                *pending
            }
            PendingAuthorizationTake::Absent | PendingAuthorizationTake::Taken(_) => {
                return Ok(ConsentOutcome::Unknown);
            }
        };
        let redirection = self.redirection(grant.redirect_uri.clone(), state);

        match decision {
            ConsentDecision::Approved => {
                let code = random_token();

                self.grants
                    .issue_code(TokenDigest::of(&code), IssuedCode::issued_at(now, grant))
                    .await
                    .map_err(ProviderError::IssueCode)
                    .map(|()| ConsentOutcome::Redirected(redirection.code(&code)))
            }
            ConsentDecision::Denied => Ok(ConsentOutcome::Redirected(
                redirection.error(AuthorizationError::AccessDenied),
            )),
        }
    }

    fn redirection(&self, redirect_uri: Url, state: Option<String>) -> Redirection {
        Redirection {
            issuer: self.issuance.issuer,
            redirect_uri,
            state,
        }
    }
}
