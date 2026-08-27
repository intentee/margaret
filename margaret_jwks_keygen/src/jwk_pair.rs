use crate::ec_jwk_public::EcJwkPublic;
use crate::jwk_signing::JwkSigning;

#[derive(Clone)]
pub struct JwkPair {
    pub public: EcJwkPublic,
    pub signing: JwkSigning,
}
