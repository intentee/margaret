use serde::Deserialize;
use serde::Serialize;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::key_use::KeyUse;

use crate::key_id::KeyId;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RsaJwk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alg: Option<JwsAlgorithm>,
    pub e: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<KeyId>,
    #[serde(default, rename = "use", skip_serializing_if = "Option::is_none")]
    pub key_use: Option<KeyUse>,
    pub n: String,
}
