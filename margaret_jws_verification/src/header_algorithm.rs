use serde::Deserialize;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum HeaderAlgorithm {
    Supported(JwsAlgorithm),
    Unsupported(String),
}
