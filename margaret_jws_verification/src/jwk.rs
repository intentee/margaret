use serde::Deserialize;
use serde::Serialize;

use crate::ec_jwk::EcJwk;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kty")]
pub enum Jwk {
    #[serde(rename = "EC")]
    Ec(EcJwk),
}
