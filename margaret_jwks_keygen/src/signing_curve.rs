use serde::Deserialize;
use serde::Serialize;

use margaret_jose_parameters::curve::Curve;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SigningCurve {
    #[serde(rename = "P-256")]
    P256,
    #[serde(rename = "P-384")]
    P384,
}

impl SigningCurve {
    #[must_use]
    pub fn curve(self) -> Curve {
        match self {
            Self::P256 => Curve::P256,
            Self::P384 => Curve::P384,
        }
    }
}
