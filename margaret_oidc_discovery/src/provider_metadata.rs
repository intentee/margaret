use serde::Deserialize;
use url::Url;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::provider_metadata_parsing::ProviderMetadataParsing;
use crate::provider_metadata_rejection::ProviderMetadataRejection;

const JWKS_URI_SCHEME: &str = "https";

#[derive(Deserialize)]
struct ProviderMetadataDocument {
    issuer: String,
    jwks_uri: String,
}

pub struct ProviderMetadata {
    jwks_uri: Url,
}

impl ProviderMetadata {
    #[must_use]
    pub fn parse(document: &[u8], issuer: &IssuerIdentifier) -> ProviderMetadataParsing {
        let ProviderMetadataDocument {
            issuer: declared_issuer,
            jwks_uri,
        } = match serde_json::from_slice(document) {
            Ok(document) => document,
            Err(source) => {
                return ProviderMetadataParsing::Rejected(ProviderMetadataRejection::Malformed {
                    source,
                });
            }
        };

        if declared_issuer != issuer.as_str() {
            return ProviderMetadataParsing::Rejected(ProviderMetadataRejection::IssuerMismatch {
                expected: issuer.clone(),
                found: declared_issuer,
            });
        }

        let jwks_uri = match Url::parse(&jwks_uri) {
            Ok(jwks_uri) => jwks_uri,
            Err(source) => {
                return ProviderMetadataParsing::Rejected(
                    ProviderMetadataRejection::JwksUriMalformed { source },
                );
            }
        };

        if jwks_uri.scheme() != JWKS_URI_SCHEME {
            return ProviderMetadataParsing::Rejected(ProviderMetadataRejection::JwksUriNotHttps {
                scheme: jwks_uri.scheme().to_string(),
            });
        }

        ProviderMetadataParsing::Accepted(Self { jwks_uri })
    }

    #[must_use]
    pub fn jwks_uri(&self) -> &Url {
        &self.jwks_uri
    }
}
