use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::bearer_token_verification::BearerTokenVerification;

pub struct BearerTokenVerifier {
    token_trust: Arc<dyn DeclaresTokenTrust>,
    token_type: TypeHeaderExpectation,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl BearerTokenVerifier {
    #[must_use]
    pub fn new_for_access_tokens(
        token_trust: Arc<dyn DeclaresTokenTrust>,
        verification_key_set_holder: VerificationKeySetHolder,
    ) -> Self {
        Self {
            token_trust,
            token_type: TypeHeaderExpectation::Required(JwtType::AccessToken),
            verification_key_set_holder,
        }
    }

    #[must_use]
    pub fn new_for_id_tokens(
        token_trust: Arc<dyn DeclaresTokenTrust>,
        verification_key_set_holder: VerificationKeySetHolder,
    ) -> Self {
        Self {
            token_trust,
            token_type: TypeHeaderExpectation::Optional(JwtType::Jwt),
            verification_key_set_holder,
        }
    }

    #[must_use]
    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        authorization: &RequestAuthorization,
        now: NumericDate,
    ) -> BearerTokenVerification<TClaims> {
        let token = match authorization {
            RequestAuthorization::Absent => return BearerTokenVerification::Absent,
            RequestAuthorization::Bearer(token) => token,
            RequestAuthorization::Malformed => {
                return BearerTokenVerification::MalformedAuthorization;
            }
            RequestAuthorization::OtherScheme => return BearerTokenVerification::NotBearer,
        };
        let Some(key_set) = self.verification_key_set_holder.get() else {
            return BearerTokenVerification::Unavailable;
        };
        let token_trust = self.token_trust.token_trust();

        match verify_serialized_jwt(
            &key_set,
            token.as_str(),
            &JwtExpectation {
                audience: &token_trust.audience,
                issuer: &token_trust.issuer,
                token_type: self.token_type,
            },
            now,
        ) {
            JwtVerification::Rejected(rejection) => BearerTokenVerification::Rejected(rejection),
            JwtVerification::Verified(verified) => BearerTokenVerification::Verified(verified),
        }
    }
}
