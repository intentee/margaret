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
pub struct JwkPublicSet {
    pub keys: Vec<JwkPublic>,
}

impl JwkPublicSet {
    #[must_use]
    pub fn find_by_kid(&self, kid: &str) -> Option<&JwkPublic> {
        self.keys.iter().find(|key| key.kid == kid)
    }
}

impl From<Arc<JwksSecret>> for JwkPublicSet {
    fn from(jwks_secret: Arc<JwksSecret>) -> Self {
        let mut keys = vec![jwks_secret.current.public.clone()];

        if jwks_secret.current.signing.kid != jwks_secret.previous.signing.kid {
            keys.push(jwks_secret.previous.public.clone());
        }

        Self { keys }
    }
}

impl From<JwksSecret> for JwkPublicSet {
    fn from(jwks_secret: JwksSecret) -> Self {
        Self::from(Arc::new(jwks_secret))
    }
}

impl VerifiesToken for JwkPublicSet {
    fn verify<TClaims: DeserializeOwned>(
        &self,
        token: &str,
    ) -> Result<TokenVerification<TClaims>, JwksKeyError> {
        let compact_jws = CompactJws::parse(token)?;
        let header: JwsHeader = serde_json::from_slice(&compact_jws.header_bytes)
            .map_err(|source| JwksKeyError::HeaderJson { source })?;

        match self.find_by_kid(&header.kid) {
            Some(key) => key.verify(token),
            None => Err(JwksKeyError::UnknownKeyId { kid: header.kid }),
        }
    }
}
