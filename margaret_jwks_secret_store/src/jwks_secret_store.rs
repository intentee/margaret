use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::map::Entry;
use uuid::Uuid;

use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::access_token_stamp::AccessTokenStamp;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_identity_session::refresh_token_claims_signed::RefreshTokenClaimsSigned;
use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;

use crate::access_token_signing::AccessTokenSigning;
use crate::jwks_secret_store_error::JwksSecretStoreError;

pub struct JwksSecretStore {
    holder: JwksSecretHolder,
}

impl JwksSecretStore {
    #[must_use]
    pub fn new(holder: JwksSecretHolder) -> Self {
        Self { holder }
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn issue_refresh_token(
        &self,
        subject: Uuid,
        now: DateTime<Utc>,
    ) -> Result<RefreshTokenClaimsSigned, JwksSecretStoreError> {
        let secret = self.current_secret()?;
        let registered = RegisteredClaims::issued_at(now, REFRESH_TOKEN_LIFETIME_SECS);
        let claims = RefreshTokenClaims {
            jti: Uuid::new_v4(),
            sub: subject,
        };

        Ok(RefreshTokenClaimsSigned {
            exp: registered.exp.seconds_since_epoch(),
            signed_claims: secret
                .current()
                .sign_json(&claims.to_payload(&registered), JwtType::Jwt),
        })
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn mint_access_token(
        &self,
        refresh_token: &str,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenMinting, JwksSecretStoreError> {
        let secret = self.current_secret()?;

        Ok(mint_access_token(&secret, refresh_token, now))
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll, and
    /// `JwksSecretStoreError::ClaimsSerialization` when the claims cannot be serialized.
    pub fn sign_access_token<TClaims: Serialize>(
        &self,
        claims: &TClaims,
        now: DateTime<Utc>,
    ) -> Result<AccessTokenSigning, JwksSecretStoreError> {
        let secret = self.current_secret()?;
        let Value::Object(application_claims) = serde_json::to_value(claims)
            .map_err(|source| JwksSecretStoreError::ClaimsSerialization { source })?
        else {
            return Ok(AccessTokenSigning::ClaimsNotAnObject);
        };
        let stamp = AccessTokenStamp::issued_at(now);
        let mut payload = stamp.to_json();

        for (member, value) in application_claims {
            match payload.entry(member) {
                Entry::Occupied(occupied) => {
                    return Ok(AccessTokenSigning::CollidingClaim {
                        member: occupied.key().clone(),
                    });
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(value);
                }
            }
        }

        Ok(AccessTokenSigning::Signed(AccessTokenClaimsSigned {
            exp: stamp.registered.exp.seconds_since_epoch(),
            signed_claims: secret
                .current()
                .sign_json(&Value::Object(payload), JwtType::AccessToken),
        }))
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn verify_access_token<TClaims: DeserializeOwned>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<JwksSecretVerificationResult<TClaims>, JwksSecretStoreError> {
        Ok(self.current_secret()?.verify_jwt(
            token,
            TypeHeaderExpectation::Required(JwtType::AccessToken),
            NumericDate::from(now),
        ))
    }

    fn current_secret(&self) -> Result<Arc<JwksSecret>, JwksSecretStoreError> {
        self.holder
            .get()
            .ok_or(JwksSecretStoreError::SecretUnavailable)
    }
}
