use thiserror::Error;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[derive(Debug, Error)]
pub enum IssuerDirectoryError {
    #[error(
        "the issuer '{issuer}' is trusted through a jwks endpoint and by another declaration, so its key set has no single source"
    )]
    JwksEndpointIssuerTrustedTwice { issuer: IssuerIdentifier },
}
