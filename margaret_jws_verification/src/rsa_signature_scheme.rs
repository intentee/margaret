use aws_lc_rs::signature::RSA_PKCS1_2048_8192_SHA256;
use aws_lc_rs::signature::RSA_PKCS1_2048_8192_SHA384;
use aws_lc_rs::signature::RSA_PKCS1_2048_8192_SHA512;
use aws_lc_rs::signature::RSA_PSS_2048_8192_SHA256;
use aws_lc_rs::signature::RSA_PSS_2048_8192_SHA384;
use aws_lc_rs::signature::RSA_PSS_2048_8192_SHA512;
use aws_lc_rs::signature::RsaParameters;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RsaSignatureScheme {
    Pkcs1Sha256,
    Pkcs1Sha384,
    Pkcs1Sha512,
    PssSha256,
    PssSha384,
    PssSha512,
}

impl RsaSignatureScheme {
    pub(crate) fn algorithm(self) -> JwsAlgorithm {
        match self {
            Self::Pkcs1Sha256 => JwsAlgorithm::Rs256,
            Self::Pkcs1Sha384 => JwsAlgorithm::Rs384,
            Self::Pkcs1Sha512 => JwsAlgorithm::Rs512,
            Self::PssSha256 => JwsAlgorithm::Ps256,
            Self::PssSha384 => JwsAlgorithm::Ps384,
            Self::PssSha512 => JwsAlgorithm::Ps512,
        }
    }

    pub(crate) fn parameters(self) -> &'static RsaParameters {
        match self {
            Self::Pkcs1Sha256 => &RSA_PKCS1_2048_8192_SHA256,
            Self::Pkcs1Sha384 => &RSA_PKCS1_2048_8192_SHA384,
            Self::Pkcs1Sha512 => &RSA_PKCS1_2048_8192_SHA512,
            Self::PssSha256 => &RSA_PSS_2048_8192_SHA256,
            Self::PssSha384 => &RSA_PSS_2048_8192_SHA384,
            Self::PssSha512 => &RSA_PSS_2048_8192_SHA512,
        }
    }
}
