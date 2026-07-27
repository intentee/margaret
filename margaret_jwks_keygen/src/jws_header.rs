use serde::Deserialize;

use crate::jws_algorithm::JwsAlgorithm;
use crate::jws_type::JwsType;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct JwsHeader {
    pub(crate) alg: JwsAlgorithm,
    pub(crate) kid: String,
    #[serde(rename = "typ")]
    pub(crate) _typ: JwsType,
}
