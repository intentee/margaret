use chrono::DateTime;
use chrono::Utc;
use serde::de::DeserializeOwned;

use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::access_token_verification::AccessTokenVerification;
use crate::verification_key_set_holder::VerificationKeySetHolder;

pub struct PublicJwksVerifier {
    verification_key_set_holder: VerificationKeySetHolder,
}

impl PublicJwksVerifier {
    #[must_use]
    pub fn new(verification_key_set_holder: VerificationKeySetHolder) -> Self {
        Self {
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

        match verify_jwt(&key_set, token, NumericDate::from(now)) {
            JwtVerification::Rejected(rejection) => AccessTokenVerification::Rejected(rejection),
            JwtVerification::Verified(verified) => AccessTokenVerification::Verified(verified),
        }
    }
}
