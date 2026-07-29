use serde::Deserialize;
use serde::Serialize;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Curve {
    #[serde(rename = "P-256")]
    P256,
    #[serde(rename = "P-384")]
    P384,
}

impl Curve {
    #[must_use]
    pub fn algorithm(self) -> JwsAlgorithm {
        match self {
            Self::P256 => JwsAlgorithm::Es256,
            Self::P384 => JwsAlgorithm::Es384,
        }
    }

    #[must_use]
    pub fn coordinate_bytes(self) -> usize {
        match self {
            Self::P256 => p256::FieldBytes::default().len(),
            Self::P384 => p384::FieldBytes::default().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Curve;
    use crate::jws_algorithm::JwsAlgorithm;

    #[test]
    fn p256_maps_to_es256() {
        assert_eq!(Curve::P256.algorithm(), JwsAlgorithm::Es256);
    }

    #[test]
    fn p384_maps_to_es384() {
        assert_eq!(Curve::P384.algorithm(), JwsAlgorithm::Es384);
    }
}
