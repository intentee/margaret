use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::Utc;
use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_clients::AcceptedClients;
use margaret_authorization_grants::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_database::database::Database;
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
    database: Arc<Database>,
    resources: &'static [&'static str],
    secret_store: Arc<JwksSecretStore>,
}

impl RevocationEndpoint {
    #[must_use]
    pub fn create(
        clients: Arc<AcceptedClients>,
        secret_store: Arc<JwksSecretStore>,
        database: Arc<Database>,
        resources: &'static [&'static str],
    ) -> Self {
        Self {
            clients,
            database,
            resources,
            secret_store,
        }
    }

    /// # Errors
    ///
    /// Returns `ProviderError::ClientAuthentication` when the client assertion cannot be
    /// remembered, and `ProviderError::AuthorizationGrants` when the application cannot reach the
    /// refresh token families.
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

        match RefreshTokenRecord::lookup(&self.database, TokenDigest::of(&token))
            .await
            .map_err(ProviderError::AuthorizationGrants)?
        {
            RefreshTokenLookup::Current { family, record }
                if record.client_id == client.client_id =>
            {
                RefreshFamilyRevocationRecord::revoke(&self.database, family, instant)
                    .await
                    .map_err(ProviderError::AuthorizationGrants)
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
