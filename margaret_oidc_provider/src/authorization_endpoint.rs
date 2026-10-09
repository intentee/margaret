use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use url::Url;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_authorization_grants::authorization_grant::AuthorizationGrant;
use margaret_http::response::Response;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_issuance::token_issuance::TokenIssuance;
use margaret_validation::validation_result::ValidationResult;

use crate::authorization_conclusion::AuthorizationConclusion;
use crate::authorization_error::AuthorizationError;
use crate::authorization_outcome::AuthorizationOutcome;
use crate::authorization_parameters::AuthorizationParameters;
use crate::authorization_request::AuthorizationRequest;
use crate::end_user_authentication::EndUserAuthentication;
use crate::frame_denied::frame_denied;
use crate::prompt::Prompt;
use crate::provider_error::ProviderError;
use crate::redirection::Redirection;

fn registered_redirect_uri(redirect_uris: &[String], requested: Option<&str>) -> Option<Url> {
    requested
        .and_then(|redirect_uri| Url::parse(redirect_uri).ok())
        .filter(|redirect_uri| {
            redirect_uris
                .iter()
                .any(|registered| registered == redirect_uri.as_str())
        })
}

fn rejected(reason: &str) -> AuthorizationOutcome {
    AuthorizationOutcome::Rejected(frame_denied(Response::text(400, reason)))
}

pub struct AuthorizationEndpoint {
    authorization: &'static str,
    clients: Arc<AcceptedClients>,
    issuance: TokenIssuance,
}

impl AuthorizationEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        authorization: &'static str,
        issuance: TokenIssuance,
    ) -> Self {
        Self {
            authorization,
            clients,
            issuance,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::IssueCode` or `ProviderError::HoldPendingAuthorization` when the
    /// application cannot store the code or the pending consent.
    pub async fn authorize(
        &self,
        request: ValidationResult<AuthorizationRequest>,
        end_user: &EndUserAuthentication,
    ) -> Result<AuthorizationOutcome, ProviderError> {
        let ValidationResult::Valid(request) = request else {
            return Ok(rejected("The authorization request is malformed"));
        };
        let Some(registered) = request
            .client_id
            .as_deref()
            .and_then(|client_id| self.clients.find(client_id))
        else {
            return Ok(rejected("The client is not accepted"));
        };
        let RegisteredClient {
            client,
            code_grant:
                RegisteredCodeGrant::Granted {
                    database,
                    policy,
                    redirect_uris,
                },
            ..
        } = registered
        else {
            return Ok(rejected("The client may not request authorization codes"));
        };
        let now = Utc::now();
        let Some(redirect_uri) =
            registered_redirect_uri(redirect_uris, request.redirect_uri.as_deref())
        else {
            return Ok(rejected(
                "The redirect uri is not registered for the client",
            ));
        };
        let redirection = Redirection {
            issuer: self.issuance.issuer,
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
                return Ok(AuthorizationOutcome::Redirected(redirection.error(&error)));
            }
            ControlFlow::Continue(parameters) => parameters,
        };
        let end_user = match end_user {
            EndUserAuthentication::Authenticated(end_user)
                if !prompt.forces_login()
                    && authentication_age.admits(end_user.authenticated_at, now) =>
            {
                *end_user
            }
            EndUserAuthentication::Anonymous | EndUserAuthentication::Authenticated(_) => {
                return Ok(match prompt {
                    Prompt::NoInteraction => AuthorizationOutcome::Redirected(
                        redirection.error(&AuthorizationError::LoginRequired),
                    ),
                    Prompt::Consent
                    | Prompt::Interactive
                    | Prompt::Login
                    | Prompt::LoginAndConsent => AuthorizationOutcome::AuthenticationRequired {
                        return_to: request.continued_at(
                            self.authorization,
                            authentication_age.after_login().as_deref(),
                            prompt.after_login(),
                        ),
                    },
                });
            }
        };

        let grant = AuthorizationGrant {
            auth_time: end_user.authenticated_at,
            client_id: client.client_id.to_string(),
            code_challenge,
            nonce,
            redirect_uri: redirection.redirect_uri.clone(),
            scopes,
            subject: end_user.subject,
        };

        AuthorizationConclusion {
            client_id: client.client_id,
            consent: policy.consent,
            grant,
            now: NumericDate::from(now),
            prompt,
            redirection,
        }
        .concluded(database)
        .await
    }
}
