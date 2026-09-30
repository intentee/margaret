use serde::Deserialize;
use serde::Serialize;

use crate::jws_algorithm::JwsAlgorithm;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Curve {
    #[serde(rename = "P-256")]
    P256,
    #[serde(rename = "P-384")]
    P384,
    #[serde(rename = "P-521")]
    P521,
}

impl Curve {
    #[must_use]
    pub fn algorithm(self) -> JwsAlgorithm {
        match self {
            Self::P256 => JwsAlgorithm::Es256,
            Self::P384 => JwsAlgorithm::Es384,
            Self::P521 => JwsAlgorithm::Es512,
        }
    }

    #[must_use]
    pub fn coordinate_octets(self) -> usize {
        match self {
            Self::P256 => 32,
            Self::P384 => 48,
            Self::P521 => 66,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Curve;
    use crate::jws_algorithm::JwsAlgorithm;

    #[test]
    fn maps_every_curve_to_its_ecdsa_algorithm() {
        assert_eq!(
            [Curve::P256, Curve::P384, Curve::P521].map(Curve::algorithm),
            [
                JwsAlgorithm::Es256,
                JwsAlgorithm::Es384,
                JwsAlgorithm::Es512
            ]
        );
    }

    #[test]
    fn sizes_every_coordinate_to_the_field_of_its_curve() {
        assert_eq!(
            [Curve::P256, Curve::P384, Curve::P521].map(Curve::coordinate_octets),
            [32, 48, 66]
        );
    }
}
