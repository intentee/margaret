use crate::key_id::KeyId;
use crate::verification_material::VerificationMaterial;

#[derive(Clone)]
pub struct VerificationKey {
    pub(crate) kid: KeyId,
    pub(crate) material: VerificationMaterial,
}

impl VerificationKey {
    #[must_use]
    pub fn new(kid: KeyId, material: VerificationMaterial) -> Self {
        Self { kid, material }
    }
}
