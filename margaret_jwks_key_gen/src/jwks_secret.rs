use serde::de::DeserializeOwned;

use crate::curve::Curve;
use crate::generate_keypair_random_kid::generate_keypair_random_kid;
use crate::jwk_pair::JwkPair;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret_verification_result::JwksSecretVerificationResult;
use crate::token_verification::TokenVerification;
use crate::verifies_any_token::VerifiesAnyToken;
use crate::verifies_token::VerifiesToken;

#[derive(Clone)]
pub struct JwksSecret {
    pub current: JwkPair,
    pub previous: JwkPair,
}

impl JwksSecret {
    pub fn fresh(crv: Curve) -> Result<Self, JwksKeyError> {
        generate_keypair_random_kid(crv).map(|jwk_pair| Self {
            current: jwk_pair.clone(),
            previous: jwk_pair,
        })
    }

    pub fn rotate(&self) -> Result<Self, JwksKeyError> {
        generate_keypair_random_kid(self.current.signing.crv).map(|current| Self {
            current,
            previous: self.current.clone(),
        })
    }
}

impl VerifiesAnyToken for JwksSecret {
    fn verify_any<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<JwksSecretVerificationResult<TClaims>, JwksKeyError> {
        match self.current.public.verify::<TClaims>(token)? {
            TokenVerification::Verified(claims) => {
                Ok(JwksSecretVerificationResult::SignedWithCurrent(claims))
            }
            TokenVerification::SignatureMismatch => {
                match self.previous.public.verify::<TClaims>(token)? {
                    TokenVerification::Verified(claims) => {
                        Ok(JwksSecretVerificationResult::SignedWithPrevious(claims))
                    }
                    TokenVerification::SignatureMismatch => {
                        Ok(JwksSecretVerificationResult::Invalid)
                    }
                }
            }
        }
    }
}
