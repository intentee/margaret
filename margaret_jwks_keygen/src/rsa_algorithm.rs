use serde::Deserialize;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum RsaAlgorithm {
    #[serde(rename = "RS256")]
    Rs256,
}

impl RsaAlgorithm {
    #[must_use]
    pub fn algorithm(self) -> JwsAlgorithm {
        match self {
            Self::Rs256 => JwsAlgorithm::Rs256,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RsaAlgorithm;
    use crate::jws_algorithm::JwsAlgorithm;

    #[test]
    fn rs256_maps_to_rs256() {
        assert_eq!(RsaAlgorithm::Rs256.algorithm(), JwsAlgorithm::Rs256);
    }
}
