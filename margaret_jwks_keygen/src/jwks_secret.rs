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
    pub next: JwkPair,
    pub previous: JwkPair,
}

impl JwksSecret {
    pub fn fresh(crv: Curve) -> Result<Self, JwksKeyError> {
        generate_keypair_random_kid(crv).and_then(|current| {
            generate_keypair_random_kid(crv).map(|next| Self {
                current: current.clone(),
                next,
                previous: current,
            })
        })
    }

    pub fn rotate(&self) -> Result<Self, JwksKeyError> {
        generate_keypair_random_kid(self.current.signing.crv).map(|next| Self {
            current: self.next.clone(),
            next,
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
            TokenVerification::Malformed(malformation) => {
                Ok(JwksSecretVerificationResult::Malformed(malformation))
            }
            TokenVerification::Verified(claims) => {
                Ok(JwksSecretVerificationResult::SignedWithCurrent(claims))
            }
            TokenVerification::SignatureMismatch => {
                match self.previous.public.verify::<TClaims>(token)? {
                    TokenVerification::Verified(claims) => {
                        Ok(JwksSecretVerificationResult::SignedWithPrevious(claims))
                    }
                    TokenVerification::Malformed(_) | TokenVerification::SignatureMismatch => {
                        Ok(JwksSecretVerificationResult::Invalid)
                    }
                }
            }
        }
    }
}
