use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_identity_session::access_token_claims_signed::AccessTokenClaimsSigned;
use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::id_token_claims::IdTokenClaims;
use margaret_identity_session::id_token_lifetime_secs::ID_TOKEN_LIFETIME_SECS;
use margaret_identity_session::issued_access_token_claims::IssuedAccessTokenClaims;
use margaret_identity_session::resource_access_token_claims::ResourceAccessTokenClaims;
use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::id_token_signing::IdTokenSigning;
use crate::jwks_secret_store_error::JwksSecretStoreError;

fn verify_attributed<TClaims: DeserializeOwned, TProfile: JwtProfile>(
    jwt: &AttributedJwt<'_>,
    key_set: &VerificationKeySet,
    now: NumericDate,
) -> JwtVerification<TClaims, TProfile> {
    match jwt.profile::<TProfile>() {
        JwtProfiling::Profiled(profiled) => profiled.verify(key_set, now),
        JwtProfiling::Rejected(rejection) => {
            JwtVerification::Rejected(JwtRejection::Type(rejection))
        }
    }
}

pub struct JwksSecretStore {
    issuance: TokenIssuance,
    secrets: Arc<JwksSecretHolder>,
}

impl JwksSecretStore {
    #[must_use]
    pub fn create(secrets: Arc<JwksSecretHolder>, issuance: TokenIssuance) -> Self {
        Self { issuance, secrets }
    }

    #[must_use]
    pub fn issue_access_token(
        &self,
        access: &IssuedAccessTokenClaims,
        audience: AudienceClaim,
        now: DateTime<Utc>,
    ) -> AccessTokenClaimsSigned {
        let registered = self
            .issuance
            .identified_claims(audience, now, ACCESS_TOKEN_LIFETIME_SECS);

        AccessTokenClaimsSigned {
            exp: registered.exp.seconds_since_epoch(),
            signed_claims: self
                .secrets
                .get()
                .current()
                .sign_json(&access.to_payload(&registered), JwtType::AccessToken),
        }
    }

    /// # Errors
    ///
    /// Returns `JwksSecretStoreError::IdTokenSigning` when the rsa key cannot sign the id token.
    pub fn issue_id_token(
        &self,
        claims: &IdTokenClaims,
        signing: IdTokenSigning,
        now: DateTime<Utc>,
    ) -> Result<String, JwksSecretStoreError> {
        let payload = claims.to_payload(&self.issuance.registered_claims(
            AudienceClaim::Single(claims.client_id.to_string()),
            now,
            ID_TOKEN_LIFETIME_SECS,
        ));
        let secret = self.secrets.get();

        match signing {
            IdTokenSigning::EllipticCurve => Ok(secret.current().sign_json(&payload, JwtType::Jwt)),
            IdTokenSigning::Rsa => secret
                .rsa()
                .current()
                .sign_jwt(&payload)
                .map_err(JwksSecretStoreError::IdTokenSigning),
        }
    }

    #[must_use]
    pub fn issue_session_access_token(
        &self,
        claims: &SessionAccessTokenClaims,
        audience: &str,
        now: DateTime<Utc>,
    ) -> AccessTokenClaimsSigned {
        let registered = self.issuance.identified_claims(
            AudienceClaim::Single(audience.to_string()),
            now,
            ACCESS_TOKEN_LIFETIME_SECS,
        );

        AccessTokenClaimsSigned {
            exp: registered.exp.seconds_since_epoch(),
            signed_claims: self
                .secrets
                .get()
                .current()
                .sign_json(&claims.to_payload(&registered), JwtType::AccessToken),
        }
    }

    #[must_use]
    pub fn verify_session_access_token(
        &self,
        token: &str,
        audience: &str,
        now: DateTime<Utc>,
    ) -> JwtVerification<SessionAccessTokenClaims, AccessTokenProfile> {
        verify_serialized_jwt(
            self.secrets.get().token_key_set(),
            token,
            &JwtExpectation {
                audience: ExpectedAudience::One(audience),
                issuer: self.issuance.issuer,
            },
            NumericDate::from(now),
        )
    }

    #[must_use]
    pub fn verify_issued_jwt<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        now: NumericDate,
    ) -> JwtVerification<TClaims, TProfile> {
        verify_attributed(jwt, self.secrets.get().published_key_set(), now)
    }

    #[must_use]
    pub fn verify_own_jwt<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        now: NumericDate,
    ) -> JwtVerification<TClaims, TProfile> {
        verify_attributed(jwt, self.secrets.get().token_key_set(), now)
    }

    #[must_use]
    pub fn verify_resource_access_token(
        &self,
        token: &str,
        audiences: &[&str],
        now: DateTime<Utc>,
    ) -> JwtVerification<ResourceAccessTokenClaims, AccessTokenProfile> {
        verify_serialized_jwt(
            self.secrets.get().token_key_set(),
            token,
            &JwtExpectation {
                audience: ExpectedAudience::AnyOf(audiences),
                issuer: self.issuance.issuer,
            },
            NumericDate::from(now),
        )
    }
}
