use serde::Deserialize;
use serde::Serialize;

use crate::jwk_public::JwkPublic;
use crate::jwk_signing::JwkSigning;

#[derive(Clone, Deserialize, Serialize)]
pub struct JwkPair {
    pub public: JwkPublic,
    pub signing: JwkSigning,
}
