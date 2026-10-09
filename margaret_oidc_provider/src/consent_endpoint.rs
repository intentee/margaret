use std::sync::Arc;

use chrono::Utc;
use url::Url;
use uuid::Uuid;

use margaret_authorization_grants::authorization_code_record::AuthorizationCodeRecord;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::pending_authorization_record::PendingAuthorizationRecord;
use margaret_authorization_grants::pending_authorization_take::PendingAuthorizationTake;
use margaret_database::database::Database;
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
    database: Arc<Database>,
    issuance: TokenIssuance,
}

impl ConsentEndpoint {
    #[must_use]
    pub fn create(database: Arc<Database>, issuance: TokenIssuance) -> Self {
        Self { database, issuance }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::AuthorizationGrants` when the application cannot take the pending
    /// authorization or store the approved code.
    pub async fn decide(
        &self,
        id: Uuid,
        end_user: &AuthenticatedEndUser,
        decision: ConsentDecision,
    ) -> Result<ConsentOutcome, ProviderError> {
        let now = NumericDate::from(Utc::now());
        let PendingAuthorization { grant, state, .. } =
            match PendingAuthorizationRecord::take(&self.database, id)
                .await
                .map_err(ProviderError::AuthorizationGrants)?
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

                AuthorizationCodeRecord::issue(
                    &self.database,
                    TokenDigest::of(&code),
                    IssuedCode::issued_at(now, grant),
                    now,
                )
                .await
                .map_err(ProviderError::AuthorizationGrants)
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
