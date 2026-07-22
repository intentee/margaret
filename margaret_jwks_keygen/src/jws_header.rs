use serde::Deserialize;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Deserialize)]
pub(crate) struct JwsHeader {
    pub(crate) alg: JwsAlgorithm,
    pub(crate) kid: String,
}
