use p256::ecdsa;

pub(crate) enum SignatureCheck {
    Malformed(ecdsa::Error),
    Matches,
    Mismatch(ecdsa::Error),
}
