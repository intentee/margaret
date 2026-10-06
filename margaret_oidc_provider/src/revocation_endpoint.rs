use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_provider_state_storage::refresh_revocation::RefreshRevocation;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_registered_claims::audience::Audience;
use margaret_token_digest::token_digest::TokenDigest;
use margaret_validation::validation_result::ValidationResult;

use crate::authenticated_client::authenticated_client;
use crate::no_store::no_store;
use crate::oauth_error::oauth_error;
use crate::provider_error::ProviderError;
use crate::token_submission::TokenSubmission;

fn revoked_or_invalid() -> Response {
    no_store(Response::text(200, ""))
}

pub struct RevocationEndpoint {
    clients: Arc<AcceptedClients>,
    resources: BTreeSet<Audience>,
    secret_store: Arc<JwksSecretStore>,
    state: Arc<dyn StoresProviderState>,
}

impl RevocationEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        secret_store: Arc<JwksSecretStore>,
        state: Arc<dyn StoresProviderState>,
    ) -> Self {
        Self {
            resources: clients
                .clients()
                .flat_map(|client| client.resources.members().iter().cloned())
                .collect(),
            clients,
            secret_store,
            state,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::State` when the refresh token families cannot be reached.
    pub async fn respond(
        &self,
        request: &Request,
        submission: ValidationResult<TokenSubmission>,
    ) -> Result<Response, ProviderError> {
        let ValidationResult::Valid(TokenSubmission { client_id, token }) = submission else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the revocation request is malformed",
            ));
        };
        let client = match authenticated_client(&self.clients, request, client_id.as_deref()) {
            ControlFlow::Break(refusal) => return Ok(refusal),
            ControlFlow::Continue(client) => client,
        };

        if let JwtVerification::Verified(_) =
            self.secret_store
                .verify_resource_access_token(&token, &self.resources, Utc::now())
        {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::Extension("unsupported_token_type".to_string()),
                "access tokens expire instead of being revoked",
            ));
        }

        self.state
            .revoke_refresh_token(TokenDigest::of(&token), &client.client_id)
            .await
            .map_err(ProviderError::State)
            .map(|revocation| match revocation {
                RefreshRevocation::ForeignClient => oauth_error(
                    400,
                    BasicErrorResponseType::InvalidGrant,
                    "the refresh token was issued to another client",
                ),
                RefreshRevocation::Revoked | RefreshRevocation::Unknown => revoked_or_invalid(),
            })
    }
}
