use serde::Deserialize;
use serde::de::IgnoredAny;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::header_type::HeaderType;
use crate::key_id::KeyId;
use crate::parameter_value::ParameterValue;

#[derive(Deserialize)]
pub(crate) struct JwsHeader {
    pub(crate) alg: ParameterValue<JwsAlgorithm>,
    #[serde(default)]
    pub(crate) crit: Option<IgnoredAny>,
    #[serde(default)]
    pub(crate) kid: Option<KeyId>,
    #[serde(default)]
    pub(crate) typ: Option<HeaderType>,
}
