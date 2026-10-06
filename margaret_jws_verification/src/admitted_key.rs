use crate::verification_key::VerificationKey;
use crate::verification_material::VerificationMaterial;

pub(crate) enum AdmittedKey {
    Identified(VerificationKey),
    Unidentified(VerificationMaterial),
}
