use std::sync::Arc;

use uuid::Uuid;

use margaret_provider_state_storage::decided_authorization::DecidedAuthorization;
use margaret_provider_state_storage::pending_authorization::PendingAuthorization;
use margaret_provider_state_storage::pending_decision::PendingDecision;
use margaret_provider_state_storage::pending_verdict::PendingVerdict;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage::token_digest::TokenDigest;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;

use crate::authenticated_end_user::AuthenticatedEndUser;
use crate::authorization_error::AuthorizationError;
use crate::consent_decision::ConsentDecision;
use crate::consent_outcome::ConsentOutcome;
use crate::provider_error::ProviderError;
use crate::random_token::random_token;
use crate::redirection::Redirection;

pub struct ConsentEndpoint {
    issuance: Arc<dyn DeclaresTokenIssuance>,
    state: Arc<dyn StoresProviderState>,
}

impl ConsentEndpoint {
    #[must_use]
    pub fn create(
        state: Arc<dyn StoresProviderState>,
        issuance: Arc<dyn DeclaresTokenIssuance>,
    ) -> Self {
        Self { issuance, state }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::State` when the pending authorization cannot be decided.
    pub async fn decide(
        &self,
        id: Uuid,
        end_user: &AuthenticatedEndUser,
        decision: ConsentDecision,
    ) -> Result<ConsentOutcome, ProviderError> {
        let code = random_token();
        let verdict = match decision {
            ConsentDecision::Approved => PendingVerdict::Approved {
                code: TokenDigest::of(&code),
            },
            ConsentDecision::Denied => PendingVerdict::Denied,
        };

        self.state
            .decide_pending_authorization(
                id,
                PendingDecision {
                    subject: end_user.subject,
                    verdict,
                },
            )
            .await
            .map_err(ProviderError::State)
            .map(|decided| match decided {
                DecidedAuthorization::Approved(pending) => {
                    ConsentOutcome::Redirected(self.redirection(*pending).code(&code))
                }
                DecidedAuthorization::Denied(pending) => ConsentOutcome::Redirected(
                    self.redirection(*pending)
                        .error(AuthorizationError::AccessDenied),
                ),
                DecidedAuthorization::Unknown => ConsentOutcome::Unknown,
            })
    }

    fn redirection(
        &self,
        PendingAuthorization { grant, state }: PendingAuthorization,
    ) -> Redirection<'_> {
        Redirection {
            issuer: &self.issuance.token_issuance().issuer,
            redirect_uri: grant.redirect_uri,
            state,
        }
    }
}
