use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::oidc_token_rejection::OidcTokenRejection;
use crate::oidc_token_verification::OidcTokenVerification;
use crate::presented_bearer::PresentedBearer;
use crate::presented_jws::PresentedJws;
use crate::presented_token::PresentedToken;

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

    #[must_use]
    pub fn verify<TClaims: DeserializeOwned>(
        &self,
        PresentedBearer { now, token }: &PresentedBearer,
    ) -> OidcTokenVerification<TClaims> {
        let presented_jws = match token {
            PresentedToken::Absent => return OidcTokenVerification::Absent,
            PresentedToken::Bearer(presented_jws) => presented_jws,
            PresentedToken::MalformedAuthorization => {
                return OidcTokenVerification::Rejected(OidcTokenRejection::MalformedAuthorization);
            }
            PresentedToken::NotBearer => return OidcTokenVerification::NotBearer,
        };
        let Some(key_set) = self.verification_key_set_holder.get() else {
            return OidcTokenVerification::Unavailable;
        };
        let jws = match presented_jws {
            PresentedJws::Parsed(jws) => jws,
            PresentedJws::Unparseable(rejection) => {
                return OidcTokenVerification::Rejected(OidcTokenRejection::UnparseableToken(
                    Arc::clone(rejection),
                ));
            }
        };
        let token_trust = self.token_trust.token_trust();

        match verify_jwt(
            &key_set,
            jws,
            &JwtExpectation {
                audience: &token_trust.audience,
                issuer: &token_trust.issuer,
                token_type: TypeHeaderExpectation::Optional(JwtType::Jwt),
            },
            *now,
        ) {
            JwtVerification::Rejected(rejection) => {
                OidcTokenVerification::Rejected(OidcTokenRejection::Token(rejection))
            }
            JwtVerification::Verified(verified) => OidcTokenVerification::Verified(verified),
        }
    }
}
