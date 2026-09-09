use std::collections::HashSet;
use std::sync::Arc;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde::de::Error as _;

use crate::compact_jws::CompactJws;
use crate::ec_jwk_public::EcJwkPublic;
use crate::jwk_public::JwkPublic;
use crate::jwks_key_error::JwksKeyError;
use crate::jwks_secret::JwksSecret;
use crate::jws_header::JwsHeader;
use crate::token_malformation::TokenMalformation;
use crate::token_verification::TokenVerification;
use crate::verifies_token::VerifiesToken;

#[derive(Clone, Debug, Serialize)]
pub struct PublicJwks {
    keys: Vec<JwkPublic>,
}

impl PublicJwks {
    /// # Errors
    ///
    /// Returns `JwksKeyError::DuplicateKeyId` when two keys share a key id.
    pub fn new(keys: Vec<JwkPublic>) -> Result<Self, JwksKeyError> {
        let mut seen: HashSet<&str> = HashSet::with_capacity(keys.len());

        for key in &keys {
            if !seen.insert(key.kid()) {
                return Err(JwksKeyError::DuplicateKeyId {
                    kid: key.kid().to_string(),
                });
            }
        }

        Ok(Self { keys })
    }

    #[must_use]
    pub fn find_by_kid(&self, kid: &str) -> Option<&JwkPublic> {
        self.keys.iter().find(|key| key.kid() == kid)
    }

    #[must_use]
    pub fn keys(&self) -> &[JwkPublic] {
        &self.keys
    }

    fn publish(&mut self, jwk_public: &EcJwkPublic) {
        if self.find_by_kid(&jwk_public.kid).is_none() {
            self.keys.push(JwkPublic::from(jwk_public.clone()));
        }
    }
}

impl<'wire> Deserialize<'wire> for PublicJwks {
    fn deserialize<Source>(deserializer: Source) -> Result<Self, Source::Error>
    where
        Source: Deserializer<'wire>,
    {
        #[derive(Deserialize)]
        struct PublicJwksWire {
            keys: Vec<JwkPublic>,
        }

        let PublicJwksWire { keys } = PublicJwksWire::deserialize(deserializer)?;

        Self::new(keys).map_err(Source::Error::custom)
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
        let compact_jws = match CompactJws::parse(token) {
            Ok(compact_jws) => compact_jws,
            Err(malformation) => return Ok(TokenVerification::Malformed(malformation)),
        };
        let header: JwsHeader = match serde_json::from_slice(&compact_jws.header_bytes) {
            Ok(header) => header,
            Err(source) => {
                return Ok(TokenVerification::Malformed(TokenMalformation::HeaderJson(
                    source,
                )));
            }
        };

        match self.find_by_kid(&header.kid) {
            Some(key) => key.verify(token),
            None => Ok(TokenVerification::Malformed(
                TokenMalformation::UnknownKeyId { kid: header.kid },
            )),
        }
    }
}
