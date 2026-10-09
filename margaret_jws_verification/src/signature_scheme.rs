use aws_lc_rs::signature::ECDSA_P256_SHA256_FIXED;
use aws_lc_rs::signature::ECDSA_P384_SHA384_FIXED;
use aws_lc_rs::signature::ECDSA_P521_SHA512_FIXED;
use aws_lc_rs::signature::ED25519;
use aws_lc_rs::signature::VerificationAlgorithm;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::rsa_signature_scheme::RsaSignatureScheme;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SignatureScheme {
    Ecdsa(Curve),
    Ed25519,
    Rsa(RsaSignatureScheme),
}

impl SignatureScheme {
    pub(crate) fn of(algorithm: JwsAlgorithm) -> Self {
        match algorithm {
            JwsAlgorithm::Es256 => Self::Ecdsa(Curve::P256),
            JwsAlgorithm::Es384 => Self::Ecdsa(Curve::P384),
            JwsAlgorithm::Es512 => Self::Ecdsa(Curve::P521),
            JwsAlgorithm::EdDsa | JwsAlgorithm::Ed25519 => Self::Ed25519,
            JwsAlgorithm::Ps256 => Self::Rsa(RsaSignatureScheme::PssSha256),
            JwsAlgorithm::Ps384 => Self::Rsa(RsaSignatureScheme::PssSha384),
            JwsAlgorithm::Ps512 => Self::Rsa(RsaSignatureScheme::PssSha512),
            JwsAlgorithm::Rs256 => Self::Rsa(RsaSignatureScheme::Pkcs1Sha256),
            JwsAlgorithm::Rs384 => Self::Rsa(RsaSignatureScheme::Pkcs1Sha384),
            JwsAlgorithm::Rs512 => Self::Rsa(RsaSignatureScheme::Pkcs1Sha512),
        }
    }

    pub(crate) fn algorithm(self) -> JwsAlgorithm {
        match self {
            Self::Ecdsa(curve) => curve.algorithm(),
            Self::Ed25519 => JwsAlgorithm::Ed25519,
            Self::Rsa(scheme) => scheme.algorithm(),
        }
    }

    pub(crate) fn verification_algorithm(self) -> &'static dyn VerificationAlgorithm {
        match self {
            Self::Ecdsa(Curve::P256) => &ECDSA_P256_SHA256_FIXED,
            Self::Ecdsa(Curve::P384) => &ECDSA_P384_SHA384_FIXED,
            Self::Ecdsa(Curve::P521) => &ECDSA_P521_SHA512_FIXED,
            Self::Ed25519 => &ED25519,
            Self::Rsa(scheme) => scheme.parameters(),
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::SignatureScheme;

    #[test]
    fn names_every_algorithm_by_the_scheme_it_verifies() {
        for algorithm in JwsAlgorithm::ALL {
            let canonical = SignatureScheme::of(algorithm).algorithm();

            assert_eq!(
                SignatureScheme::of(canonical),
                SignatureScheme::of(algorithm)
            );
        }
    }

    #[test]
    fn verifies_eddsa_as_the_ed25519_scheme() {
        assert_eq!(
            SignatureScheme::of(JwsAlgorithm::EdDsa).algorithm(),
            JwsAlgorithm::Ed25519
        );
    }
}
