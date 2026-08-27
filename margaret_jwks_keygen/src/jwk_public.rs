use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;
use serde::de::DeserializeOwned;

use crate::ec_jwk_public::EcJwkPublic;
use crate::jwks_key_error::JwksKeyError;
use crate::rsa_jwk_public::RsaJwkPublic;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kty")]
pub enum JwkPublic {
    #[serde(rename = "EC")]
    Ec(EcJwkPublic),
    #[serde(rename = "RSA")]
    Rsa(RsaJwkPublic),
}

impl JwkPublic {
    #[must_use]
    pub fn kid(&self) -> &str {
        match self {
            Self::Ec(key) => &key.kid,
            Self::Rsa(key) => &key.kid,
        }
    }
}

impl From<EcJwkPublic> for JwkPublic {
    fn from(key: EcJwkPublic) -> Self {
        Self::Ec(key)
    }
}

impl Serialize for JwkPublic {
    fn serialize<Target>(&self, serializer: Target) -> Result<Target::Ok, Target::Error>
    where
        Target: Serializer,
    {
        match self {
            Self::Ec(key) => key.serialize(serializer),
            Self::Rsa(key) => key.serialize(serializer),
        }
    }
}

impl VerifiesToken for JwkPublic {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        match self {
            Self::Ec(key) => key.verify(token),
            Self::Rsa(key) => key.verify(token),
        }
    }
}
