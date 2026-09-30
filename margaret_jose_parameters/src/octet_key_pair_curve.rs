use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum OctetKeyPairCurve {
    Ed25519,
}
