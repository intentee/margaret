use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use url::Url;
use uuid::Uuid;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients::code_grant_policy::CodeGrantPolicy;
use margaret_accepted_clients::consent_policy::ConsentPolicy;
use margaret_http::response::Response;
use margaret_provider_state_storage::authorization_grant::AuthorizationGrant;
use margaret_provider_state_storage::pending_authorization::PendingAuthorization;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_token_digest::random_token::random_token;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_validation::validation_result::ValidationResult;

use crate::authorization_error::AuthorizationError;
use crate::authorization_outcome::AuthorizationOutcome;
use crate::authorization_parameters::AuthorizationParameters;
use crate::authorization_request::AuthorizationRequest;
use crate::consent_request::ConsentRequest;
use crate::end_user_authentication::EndUserAuthentication;
use crate::frame_denied::frame_denied;
use crate::prompt::Prompt;
use crate::provider_endpoints::ProviderEndpoints;
use crate::provider_error::ProviderError;
use crate::redirection::Redirection;

fn registered_redirect_uri(policy: &CodeGrantPolicy, requested: Option<&str>) -> Option<Url> {
    requested
        .and_then(|redirect_uri| Url::parse(redirect_uri).ok())
        .filter(|redirect_uri| policy.redirect_uris.members().contains(redirect_uri))
}

fn rejected(reason: &str) -> AuthorizationOutcome {
    AuthorizationOutcome::Rejected(frame_denied(Response::text(400, reason)))
}

pub struct AuthorizationEndpoint {
    clients: Arc<AcceptedClients>,
    endpoints: Arc<ProviderEndpoints>,
    issuance: Arc<dyn DeclaresTokenIssuance>,
    state: Arc<dyn StoresProviderState>,
}

impl AuthorizationEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        state: Arc<dyn StoresProviderState>,
        endpoints: Arc<ProviderEndpoints>,
        issuance: Arc<dyn DeclaresTokenIssuance>,
    ) -> Self {
        Self {
            clients,
            endpoints,
            issuance,
            state,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::State` when the code or the pending consent cannot be stored.
    pub async fn authorize(
        &self,
        request: ValidationResult<AuthorizationRequest>,
        end_user: &EndUserAuthentication,
    ) -> Result<AuthorizationOutcome, ProviderError> {
        let ValidationResult::Valid(request) = request else {
            return Ok(rejected("The authorization request is malformed"));
        };
        let Some(client) = request
            .client_id
            .as_deref()
            .and_then(|client_id| self.clients.find(client_id))
        else {
            return Ok(rejected("The client is not accepted"));
        };
        let AuthorizationCodeGrant::Granted(policy) = &client.authorization_code else {
            return Ok(rejected("The client may not request authorization codes"));
        };
        let Some(redirect_uri) = registered_redirect_uri(policy, request.redirect_uri.as_deref())
        else {
            return Ok(rejected(
                "The redirect uri is not registered for the client",
            ));
        };
        let redirection = Redirection {
            issuer: &self.issuance.token_issuance().issuer,
            redirect_uri,
            state: request.state.clone(),
        };
        let AuthorizationParameters {
            authentication_age,
            code_challenge,
            nonce,
            prompt,
            scopes,
        } = match AuthorizationParameters::of(policy, &request) {
            ControlFlow::Break(error) => {
                return Ok(AuthorizationOutcome::Redirected(redirection.error(error)));
            }
            ControlFlow::Continue(parameters) => parameters,
        };
        let end_user = match end_user {
            EndUserAuthentication::Authenticated(end_user)
                if !prompt.forces_login()
                    && authentication_age.admits(end_user.authenticated_at, Utc::now()) =>
            {
                *end_user
            }
            EndUserAuthentication::Anonymous | EndUserAuthentication::Authenticated(_) => {
                return Ok(match prompt {
                    Prompt::NoInteraction => AuthorizationOutcome::Redirected(
                        redirection.error(AuthorizationError::LoginRequired),
                    ),
                    Prompt::Consent
                    | Prompt::Interactive
                    | Prompt::Login
                    | Prompt::LoginAndConsent => AuthorizationOutcome::AuthenticationRequired {
                        return_to: request
                            .continued_at(&self.endpoints.authorization, prompt.after_login()),
                    },
                });
            }
        };

        let grant = AuthorizationGrant {
            auth_time: end_user.authenticated_at,
            client_id: client.client_id.clone(),
            code_challenge,
            nonce,
            redirect_uri: redirection.redirect_uri.clone(),
            scopes,
            subject: end_user.subject,
        };

        if policy.consent == ConsentPolicy::Implicit && !prompt.forces_consent() {
            let code = random_token();

            return self
                .state
                .issue_code(TokenDigest::of(&code), grant)
                .await
                .map_err(ProviderError::State)
                .map(|()| AuthorizationOutcome::Redirected(redirection.code(&code)));
        }

        if prompt == Prompt::NoInteraction {
            return Ok(AuthorizationOutcome::Redirected(
                redirection.error(AuthorizationError::ConsentRequired),
            ));
        }

        let id = Uuid::new_v4();
        let consent = ConsentRequest {
            client_id: grant.client_id.clone(),
            id,
            scopes: grant.scopes.clone(),
        };

        self.state
            .hold_pending_authorization(
                id,
                PendingAuthorization {
                    grant,
                    state: redirection.state,
                },
            )
            .await
            .map_err(ProviderError::State)
            .map(|()| AuthorizationOutcome::ConsentRequired(consent))
    }
}
