use thiserror::Error;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[derive(Debug, Error)]
pub enum IssuerDirectoryError {
    #[error("the issuer '{issuer}' is trusted by more than one declaration")]
    IssuerTrustedTwice { issuer: IssuerIdentifier },
}
