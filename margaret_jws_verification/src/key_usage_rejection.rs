use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use margaret_jose_parameters::key_operation::KeyOperation;
use margaret_jose_parameters::key_use::KeyUse;

#[derive(Debug)]
pub enum KeyUsageRejection {
    DuplicateOperation {
        key_op: KeyOperation,
    },
    EncryptionUse,
    MissingUseAmongEncryptionKeys,
    OperationContradictsUse {
        key_op: KeyOperation,
        key_use: KeyUse,
    },
    UnrecognizedOperation {
        key_op: String,
    },
    UnrecognizedUse {
        key_use: String,
    },
    UnrelatedOperation {
        key_op: KeyOperation,
    },
    VerificationNotPermitted {
        key_ops: Vec<KeyOperation>,
    },
}

impl Display for KeyUsageRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DuplicateOperation { key_op } => write!(
                formatter,
                "the key lists the operation '{key_op}' more than once"
            ),
            Self::EncryptionUse => write!(formatter, "the key is meant for encryption"),
            Self::MissingUseAmongEncryptionKeys => write!(
                formatter,
                "the key declares no use although the set also publishes encryption keys"
            ),
            Self::OperationContradictsUse { key_op, .. } => write!(
                formatter,
                "the key operation '{key_op}' contradicts the use the key declares"
            ),
            Self::UnrecognizedOperation { key_op } => {
                write!(
                    formatter,
                    "the key lists the unrecognized operation '{key_op}'"
                )
            }
            Self::UnrecognizedUse { key_use } => {
                write!(
                    formatter,
                    "the key declares the unrecognized use '{key_use}'"
                )
            }
            Self::UnrelatedOperation { key_op } => write!(
                formatter,
                "the key combines verification with the unrelated operation '{key_op}'"
            ),
            Self::VerificationNotPermitted { .. } => {
                write!(formatter, "the key's operations do not permit verification")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_jose_parameters::key_operation::KeyOperation;
    use margaret_jose_parameters::key_use::KeyUse;

    use super::KeyUsageRejection;

    #[test]
    fn describes_every_rejection() {
        let described = [
            KeyUsageRejection::DuplicateOperation {
                key_op: KeyOperation::Verify,
            },
            KeyUsageRejection::EncryptionUse,
            KeyUsageRejection::MissingUseAmongEncryptionKeys,
            KeyUsageRejection::OperationContradictsUse {
                key_op: KeyOperation::Encrypt,
                key_use: KeyUse::Signature,
            },
            KeyUsageRejection::UnrecognizedOperation {
                key_op: "attest".to_string(),
            },
            KeyUsageRejection::UnrecognizedUse {
                key_use: "tls".to_string(),
            },
            KeyUsageRejection::UnrelatedOperation {
                key_op: KeyOperation::Decrypt,
            },
            KeyUsageRejection::VerificationNotPermitted {
                key_ops: vec![KeyOperation::Sign],
            },
        ]
        .map(|rejection| rejection.to_string());

        assert_eq!(
            described[0],
            "the key lists the operation 'verify' more than once"
        );
        assert_eq!(described[1], "the key is meant for encryption");
        assert_eq!(
            described[2],
            "the key declares no use although the set also publishes encryption keys"
        );
        assert_eq!(
            described[3],
            "the key operation 'encrypt' contradicts the use the key declares"
        );
        assert_eq!(
            described[4],
            "the key lists the unrecognized operation 'attest'"
        );
        assert_eq!(described[5], "the key declares the unrecognized use 'tls'");
        assert_eq!(
            described[6],
            "the key combines verification with the unrelated operation 'decrypt'"
        );
        assert_eq!(
            described[7],
            "the key's operations do not permit verification"
        );
    }
}
