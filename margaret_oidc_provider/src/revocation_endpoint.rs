use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_registered_claims::numeric_date::NumericDate;
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
    grants: Arc<dyn StoresAuthorizationGrants>,
    resources: &'static [&'static str],
    secret_store: Arc<JwksSecretStore>,
}

impl RevocationEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        secret_store: Arc<JwksSecretStore>,
        grants: Arc<dyn StoresAuthorizationGrants>,
        resources: &'static [&'static str],
    ) -> Self {
        Self {
            clients,
            grants,
            resources,
            secret_store,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::ClientAuthentication` when the client assertion cannot be
    /// remembered, and `ProviderError::FindRefreshToken` or `ProviderError::RevokeRefreshFamily`
    /// when the application cannot reach the refresh token families.
    pub async fn respond(
        &self,
        request: &Request,
        submission: ValidationResult<TokenSubmission>,
    ) -> Result<Response, ProviderError> {
        let ValidationResult::Valid(TokenSubmission {
            client_authentication,
            token,
        }) = submission
        else {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidRequest,
                "the revocation request is malformed",
            ));
        };
        let now = Utc::now();
        let instant = NumericDate::from(now);
        let client =
            match authenticated_client(&self.clients, request, &client_authentication, instant)
                .await?
            {
                ControlFlow::Break(refusal) => return Ok(refusal),
                ControlFlow::Continue(registered) => &registered.client,
            };

        if let JwtVerification::Verified(_) =
            self.secret_store
                .verify_resource_access_token(&token, self.resources, now)
        {
            return Ok(oauth_error(
                400,
                BasicErrorResponseType::Extension("unsupported_token_type".to_string()),
                "access tokens expire instead of being revoked",
            ));
        }

        match self
            .grants
            .find_refresh_token(TokenDigest::of(&token))
            .await
            .map_err(ProviderError::FindRefreshToken)?
        {
            RefreshTokenLookup::Current { family, record }
                if record.client_id == client.client_id =>
            {
                self.grants
                    .revoke_refresh_family(family, instant)
                    .await
                    .map_err(ProviderError::RevokeRefreshFamily)
                    .map(|()| revoked_or_invalid())
            }
            RefreshTokenLookup::Current { .. } => Ok(oauth_error(
                400,
                BasicErrorResponseType::InvalidGrant,
                "the refresh token was issued to another client",
            )),
            RefreshTokenLookup::Superseded { .. } | RefreshTokenLookup::Unknown => {
                Ok(revoked_or_invalid())
            }
        }
    }
}
