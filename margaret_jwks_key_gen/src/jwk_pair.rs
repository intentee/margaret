use crate::jwk_public::JwkPublic;
use crate::jwk_signing::JwkSigning;

#[derive(Clone)]
pub struct JwkPair {
    pub public: JwkPublic,
    pub signing: JwkSigning,
}
