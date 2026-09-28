#[derive(Clone, PartialEq)]
pub(crate) enum SigningMaterial {
    P256(p256::ecdsa::SigningKey),
    P384(p384::ecdsa::SigningKey),
}
