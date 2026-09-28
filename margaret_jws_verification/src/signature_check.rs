use aws_lc_rs::error::Unspecified;
use p256::ecdsa;

pub(crate) enum SignatureCheck {
    EcdsaMalformed(ecdsa::Error),
    EcdsaMismatch(ecdsa::Error),
    Matches,
    RsaMismatch(Unspecified),
}
