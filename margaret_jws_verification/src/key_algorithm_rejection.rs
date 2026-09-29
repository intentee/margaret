use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

#[derive(Debug)]
pub enum KeyAlgorithmRejection {
    AlgorithmMismatch {
        declared: JwsAlgorithm,
        implied: JwsAlgorithm,
    },
    UnsupportedAlgorithm {
        alg: String,
    },
}

impl Display for KeyAlgorithmRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AlgorithmMismatch { declared, implied } => write!(
                formatter,
                "the key declares the algorithm {declared}, but its key material verifies {implied}"
            ),
            Self::UnsupportedAlgorithm { alg } => {
                write!(
                    formatter,
                    "the key declares the unsupported algorithm '{alg}'"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

    use super::KeyAlgorithmRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeyAlgorithmRejection::AlgorithmMismatch {
                declared: JwsAlgorithm::Es384,
                implied: JwsAlgorithm::Es256,
            },
            KeyAlgorithmRejection::UnsupportedAlgorithm {
                alg: "RSA-OAEP".to_string(),
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the key declares the algorithm ES384, but its key material verifies ES256"
        );
        assert_eq!(
            described[1],
            "the key declares the unsupported algorithm 'RSA-OAEP'"
        );
    }
}
