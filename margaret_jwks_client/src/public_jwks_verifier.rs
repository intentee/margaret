use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::access_token_verification::AccessTokenVerification;

pub struct PublicJwksVerifier {
    token_trust: Arc<dyn DeclaresTokenTrust>,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(
        token_trust: Arc<dyn DeclaresTokenTrust>,
        verification_key_set_holder: VerificationKeySetHolder,
    ) -> Self {
        Self {
            token_trust,
            verification_key_set_holder,
        }
    }

    #[must_use]
    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
        now: DateTime<Utc>,
    ) -> AccessTokenVerification<TClaims> {
        let Some(key_set) = self.verification_key_set_holder.get() else {
            return AccessTokenVerification::NotReady;
        };
        let token_trust = self.token_trust.token_trust();

        match verify_serialized_jwt(
            &key_set,
            token,
            &JwtExpectation {
                audience: &token_trust.audience,
                issuer: &token_trust.issuer,
                token_type: TypeHeaderExpectation::Required(JwtType::AccessToken),
            },
            NumericDate::from(now),
        ) {
            JwtVerification::Rejected(rejection) => AccessTokenVerification::Rejected(rejection),
            JwtVerification::Verified(verified) => AccessTokenVerification::Verified(verified),
        }
    }
}
