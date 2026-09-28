use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::header_algorithm::HeaderAlgorithm;
use crate::header_type::HeaderType;
use crate::key_id::KeyId;

#[derive(Deserialize)]
pub(crate) struct JwsHeader {
    pub(crate) alg: HeaderAlgorithm,
    #[serde(default)]
    pub(crate) crit: Option<IgnoredAny>,
    #[serde(default)]
    pub(crate) kid: Option<KeyId>,
    #[serde(default)]
    pub(crate) typ: Option<HeaderType>,
}
