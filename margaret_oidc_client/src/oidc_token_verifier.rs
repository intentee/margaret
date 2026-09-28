use std::sync::Arc;
use std::time::SystemTime;

use serde::de::DeserializeOwned;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::oidc_token_rejection::OidcTokenRejection;
use crate::oidc_token_verification::OidcTokenVerification;

pub struct OidcTokenVerifier {
    token_trust: Arc<dyn DeclaresTokenTrust>,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl OidcTokenVerifier {
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

    /// # Errors
    ///
    /// Returns `RegisteredClaimsError` when the system clock cannot be read as a numeric date.
    pub fn verify_authorization<TClaims: DeserializeOwned>(
        &self,
        authorization: &RequestAuthorization,
    ) -> Result<OidcTokenVerification<TClaims>, RegisteredClaimsError> {
        NumericDate::from_system_time(SystemTime::now())
            .map(|now| self.verification_at(authorization, now))
    }

    fn verification_at<TClaims: DeserializeOwned>(
        &self,
        authorization: &RequestAuthorization,
        now: NumericDate,
    ) -> OidcTokenVerification<TClaims> {
        let token = match authorization {
            RequestAuthorization::Absent => return OidcTokenVerification::Absent,
            RequestAuthorization::Bearer(token) => token,
            RequestAuthorization::Malformed => {
                return OidcTokenVerification::Rejected(OidcTokenRejection::MalformedAuthorization);
            }
            RequestAuthorization::OtherScheme => return OidcTokenVerification::NotBearer,
        };
        let Some(key_set) = self.verification_key_set_holder.get() else {
            return OidcTokenVerification::Unavailable;
        };
        let token_trust = self.token_trust.token_trust();

        match verify_jwt(
            &key_set,
            token.as_str(),
            &JwtExpectation {
                audience: &token_trust.audience,
                issuer: &token_trust.issuer,
                token_type: TypeHeaderExpectation::Optional(JwtType::Jwt),
            },
            now,
        ) {
            JwtVerification::Rejected(rejection) => {
                OidcTokenVerification::Rejected(OidcTokenRejection::Token(rejection))
            }
            JwtVerification::Verified(verified) => OidcTokenVerification::Verified(verified),
        }
    }
}
