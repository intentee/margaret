use uuid::Uuid;

use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;

use crate::authorization_error::AuthorizationError;
use crate::authorization_outcome::AuthorizationOutcome;
use crate::consent_request::ConsentRequest;
use crate::prompt::Prompt;
use crate::provider_error::ProviderError;
use crate::redirection::Redirection;

pub(crate) struct AuthorizationConclusion {
    pub(crate) client_id: &'static str,
    pub(crate) consent: ConsentPolicy,
    pub(crate) grant: AuthorizationGrant,
    pub(crate) now: NumericDate,
    pub(crate) prompt: Prompt,
    pub(crate) redirection: Redirection,
}

impl AuthorizationConclusion {
    pub(crate) async fn concluded(
        self,
        grants: &dyn StoresAuthorizationGrants,
    ) -> Result<AuthorizationOutcome, ProviderError> {
        let Self {
            client_id,
            consent,
            grant,
            now,
            prompt,
            redirection,
        } = self;

        if consent == ConsentPolicy::Implicit && !prompt.forces_consent() {
            let code = random_token();

            return grants
                .issue_code(TokenDigest::of(&code), IssuedCode::issued_at(now, grant))
                .await
                .map_err(ProviderError::IssueCode)
                .map(|()| AuthorizationOutcome::Redirected(redirection.code(&code)));
        }

        if prompt == Prompt::NoInteraction {
            return Ok(AuthorizationOutcome::Redirected(
                redirection.error(AuthorizationError::ConsentRequired),
            ));
        }

        let id = Uuid::new_v4();
        let consent = ConsentRequest {
            client_id,
            id,
            scopes: grant.scopes.clone(),
        };

        grants
            .hold_pending_authorization(
                id,
                PendingAuthorization::held_at(now, grant, redirection.state),
            )
            .await
            .map_err(ProviderError::HoldPendingAuthorization)
            .map(|()| AuthorizationOutcome::ConsentRequired(consent))
    }
}
