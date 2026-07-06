use async_trait::async_trait;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;
use crate::sign_signing_input::sign_signing_input;
use crate::signs_claims::SignsClaims;

#[derive(Clone, Deserialize, Serialize)]
pub struct JwkSigning {
    pub crv: Curve,
    pub kid: String,
    pub pem: String,
}

impl JwkSigning {
    fn header_json(&self) -> String {
        let mut header = Map::with_capacity(2);

        header.insert(
            "alg".to_string(),
            Value::String(self.crv.algorithm().wire_name().to_string()),
        );
        header.insert("kid".to_string(), Value::String(self.kid.clone()));

        Value::Object(header).to_string()
    }
}

#[async_trait]
impl SignsClaims for JwkSigning {
    async fn sign<TClaims: Send + Serialize + Sync>(
        &self,
        claims: &TClaims,
    ) -> Result<String, JwksKeyError> {
        let claims_json =
            serde_json::to_vec(claims).map_err(|source| JwksKeyError::ClaimsJson { source })?;
        let signing_input = format!(
            "{}.{}",
            Base64UrlUnpadded::encode_string(self.header_json().as_bytes()),
            Base64UrlUnpadded::encode_string(&claims_json),
        );
        let signature = sign_signing_input(&signing_input, &self.pem, self.crv)?;

        Ok(format!("{signing_input}.{signature}"))
    }
}
