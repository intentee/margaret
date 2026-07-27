use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::compact_jws::CompactJws;
use crate::jwk_public::JwkPublic;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret::JwksSecret;
use crate::jws_header::JwsHeader;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicJwks {
    pub keys: Vec<JwkPublic>,
}

impl PublicJwks {
    pub fn find_by_kid(&self, kid: &str) -> Result<Option<&JwkPublic>, JwksKeyError> {
        let mut matching = self.keys.iter().filter(|key| key.kid == kid);
        let first = matching.next();

        if matching.next().is_some() {
            return Err(JwksKeyError::DuplicateKeyId {
                kid: kid.to_string(),
            });
        }

        Ok(first)
    }

    fn publish(&mut self, jwk_public: &JwkPublic) {
        if !self.keys.iter().any(|key| key.kid == jwk_public.kid) {
            self.keys.push(jwk_public.clone());
        }
    }
}

impl From<Arc<JwksSecret>> for PublicJwks {
    fn from(jwks_secret: Arc<JwksSecret>) -> Self {
        let mut public_jwks = Self { keys: Vec::new() };

        public_jwks.publish(&jwks_secret.current.public);
        public_jwks.publish(&jwks_secret.previous.public);
        public_jwks.publish(&jwks_secret.next.public);

        public_jwks
    }
}

impl From<JwksSecret> for PublicJwks {
    fn from(jwks_secret: JwksSecret) -> Self {
        Self::from(Arc::new(jwks_secret))
    }
}

impl VerifiesToken for PublicJwks {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        let compact_jws = CompactJws::parse(token)?;
        let header: JwsHeader = serde_json::from_slice(&compact_jws.header_bytes)
            .map_err(|source| JwksKeyError::HeaderJson { source })?;

        match self.find_by_kid(&header.kid)? {
            Some(key) => key.verify(token),
            None => Err(JwksKeyError::UnknownKeyId { kid: header.kid }),
        }
    }
}

#[cfg(test)]
mod tests {
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;
    use serde_json::Value;

    use super::PublicJwks;
    use crate::curve::Curve;
    use crate::jwk_public::JwkPublic;
    use crate::key_type::KeyType;
    use crate::key_use::KeyUse;
    use crate::verifies_token::VerifiesToken;

    fn public_key(kid: &str) -> JwkPublic {
        JwkPublic {
            crv: Curve::P256,
            kid: kid.to_string(),
            kty: KeyType::Ec,
            use_: KeyUse::Signature,
            x: "coordinate".to_string(),
            y: "coordinate".to_string(),
        }
    }

    #[test]
    fn rejects_ambiguous_duplicate_key_ids() {
        let jwks = PublicJwks {
            keys: vec![public_key("duplicate"), public_key("duplicate")],
        };

        assert!(jwks.find_by_kid("duplicate").is_err());
    }

    #[test]
    fn rejects_a_token_when_its_key_id_is_ambiguous() {
        let jwks = PublicJwks {
            keys: vec![public_key("duplicate"), public_key("duplicate")],
        };
        let header =
            Base64UrlUnpadded::encode_string(br#"{"alg":"ES256","kid":"duplicate","typ":"JWT"}"#);
        let claims = Base64UrlUnpadded::encode_string(br#"{}"#);
        let token = format!("{header}.{claims}.AA");

        assert!(jwks.verify::<Value>(&token).is_err());
    }
}
