use std::collections::BTreeSet;
use std::ops::ControlFlow;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_json::map::Entry;
use uuid::Uuid;

use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::access_token_stamp::AccessTokenStamp;
use margaret_identity_session::id_token_lifetime_secs::ID_TOKEN_LIFETIME_SECS;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_identity_session::refresh_token_claims_signed::RefreshTokenClaimsSigned;
use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_identity_session::sign_in_transaction_claims::SignInTransactionClaims;
use margaret_identity_session::sign_in_transaction_lifetime_secs::SIGN_IN_TRANSACTION_LIFETIME_SECS;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::sign_in_transaction_profile::SignInTransactionProfile;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_issuance::declares_token_issuance::DeclaresTokenIssuance;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;

use crate::access_token_signing::AccessTokenSigning;
use crate::id_token_signing::IdTokenSigning;
use crate::jwks_secret_store_error::JwksSecretStoreError;
use crate::provider_identity::ProviderIdentity;
use crate::provider_tokens::ProviderTokens;

fn rsa_signed_id_token(
    signed: Result<String, JwksKeyError>,
) -> Result<String, JwksSecretStoreError> {
    signed.map_err(|source| JwksSecretStoreError::IdTokenSigning { source })
}

pub struct JwksSecretStore {
    issuance: Arc<dyn DeclaresTokenIssuance>,
    roller: Arc<JwksRoller>,
}

impl JwksSecretStore {
    #[must_use]
    pub fn create(roller: Arc<JwksRoller>, issuance: Arc<dyn DeclaresTokenIssuance>) -> Self {
        Self { issuance, roller }
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll, and
    /// `JwksSecretStoreError::IdTokenSigning` when the rsa key cannot sign the id token.
    pub fn issue_provider_tokens(
        &self,
        access: &ResourceAccessTokenClaims,
        audience: AudienceClaim,
        identity: ProviderIdentity,
        now: DateTime<Utc>,
    ) -> Result<ProviderTokens, JwksSecretStoreError> {
        let secret = self.current_secret()?;
        let issuance = self.issuance.token_issuance();
        let registered = RegisteredClaims {
            aud: audience,
            ..issuance.registered_claims(now, ACCESS_TOKEN_LIFETIME_SECS)
        };
        let access_token = AccessTokenClaimsSigned {
            exp: registered.exp.seconds_since_epoch(),
            signed_claims: secret.current().sign_json(
                &access.to_payload(&registered, Uuid::new_v4()),
                JwtType::AccessToken,
            ),
        };

        match identity {
            ProviderIdentity::Asserted { claims, signing } => {
                let payload = claims.to_payload(&RegisteredClaims {
                    aud: AudienceClaim::Single(claims.client_id.as_str().to_string()),
                    ..issuance.registered_claims(now, ID_TOKEN_LIFETIME_SECS)
                });
                let id_token = match signing {
                    IdTokenSigning::EllipticCurve => {
                        Ok(secret.current().sign_json(&payload, JwtType::Jwt))
                    }
                    IdTokenSigning::Rsa => {
                        rsa_signed_id_token(secret.rsa().current().sign_jwt(&payload))
                    }
                };

                id_token.map(|id_token| ProviderTokens {
                    access_token,
                    id_token: Some(id_token),
                })
            }
            ProviderIdentity::Withheld => Ok(ProviderTokens {
                access_token,
                id_token: None,
            }),
        }
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
        let registered = self
            .issuance
            .token_issuance()
            .registered_claims(now, REFRESH_TOKEN_LIFETIME_SECS);
        let claims = RefreshTokenClaims {
            jti: Uuid::new_v4(),
            sub: subject,
        };

        Ok(RefreshTokenClaimsSigned {
            exp: registered.exp.seconds_since_epoch(),
            signed_claims: secret
                .current()
                .sign_json(&claims.to_payload(&registered), JwtType::Refresh),
        })
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn issue_sign_in_transaction(
        &self,
        transaction: &SignInTransactionClaims,
        now: DateTime<Utc>,
    ) -> Result<String, JwksSecretStoreError> {
        let secret = self.current_secret()?;
        let registered = self
            .issuance
            .token_issuance()
            .registered_claims(now, SIGN_IN_TRANSACTION_LIFETIME_SECS);

        Ok(secret.current().sign_json(
            &transaction.to_payload(&registered),
            JwtType::SignInTransaction,
        ))
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

        Ok(mint_access_token(
            &secret,
            self.issuance.token_issuance(),
            refresh_token,
            now,
        ))
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
        let stamp = AccessTokenStamp::issued_by(self.issuance.token_issuance(), now);
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
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll, and
    /// `JwksSecretStoreError::ClaimsSerialization` when the claims cannot be serialized.
    pub fn sign_client_assertion<TClaims: Serialize>(
        &self,
        claims: &TClaims,
    ) -> Result<String, JwksSecretStoreError> {
        let secret = self.current_secret()?;
        let payload = serde_json::to_value(claims)
            .map_err(|source| JwksSecretStoreError::ClaimsSerialization { source })?;

        Ok(secret
            .current()
            .sign_json(&payload, JwtType::ClientAuthentication))
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn verify_access_token<TClaims: DeserializeOwned>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<JwksSecretVerificationResult<TClaims, AccessTokenProfile>, JwksSecretStoreError>
    {
        let issuance = self.issuance.token_issuance();

        Ok(self.current_secret()?.verify_jwt(
            token,
            &JwtExpectation {
                audience: &issuance.audience,
                issuer: &issuance.issuer,
            },
            NumericDate::from(now),
        ))
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn verify_resource_access_token(
        &self,
        token: &str,
        audiences: &BTreeSet<Audience>,
        now: DateTime<Utc>,
    ) -> Result<JwtVerification<ResourceAccessTokenClaims, AccessTokenProfile>, JwksSecretStoreError>
    {
        let secret = self.current_secret()?;
        let attributed =
            match attribute_serialized_jwt(token, &self.issuance.token_issuance().issuer) {
                ControlFlow::Continue(attributed) => attributed,
                ControlFlow::Break(rejection) => return Ok(JwtVerification::Rejected(rejection)),
            };

        Ok(match attributed.profile::<AccessTokenProfile>() {
            JwtProfiling::Profiled(profiled) => {
                profiled.verify_for_any(secret.key_set(), audiences, NumericDate::from(now))
            }
            JwtProfiling::Rejected(rejection) => {
                JwtVerification::Rejected(JwtRejection::Type(rejection))
            }
        })
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::SecretUnavailable` before the first roll.
    pub fn verify_sign_in_transaction(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> Result<
        JwtVerification<SignInTransactionClaims, SignInTransactionProfile>,
        JwksSecretStoreError,
    > {
        let issuance = self.issuance.token_issuance();

        Ok(verify_serialized_jwt(
            self.current_secret()?.key_set(),
            token,
            &JwtExpectation {
                audience: &issuance.audience,
                issuer: &issuance.issuer,
            },
            NumericDate::from(now),
        ))
    }

    fn current_secret(&self) -> Result<Arc<JwksSecret>, JwksSecretStoreError> {
        self.roller
            .jwks_secret_holder()
            .get()
            .ok_or(JwksSecretStoreError::SecretUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;

    use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

    use super::rsa_signed_id_token;

    #[test]
    fn reports_an_id_token_the_rsa_key_cannot_sign() {
        assert_eq!(
            rsa_signed_id_token(Err(JwksKeyError::RsaSigning {
                source: Unspecified
            }))
            .expect_err("the signing failure is reported")
            .to_string(),
            format!(
                "the id token could not be signed with the rsa key: the rsa signing key could not sign: {Unspecified}"
            )
        );
    }
}
