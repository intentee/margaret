use std::ops::ControlFlow;
use std::sync::Arc;

use url::Url;

use margaret_https_url::https_url::HttpsUrl;

use crate::advertised_endpoint::AdvertisedEndpoint;
use crate::authorization_response_issuer::AuthorizationResponseIssuer;
use crate::metadata_endpoint::MetadataEndpoint;
use crate::provider_metadata_document::ProviderMetadataDocument;
use crate::provider_metadata_parsing::ProviderMetadataParsing;
use crate::provider_metadata_rejection::ProviderMetadataRejection;

fn https_endpoint(
    endpoint: MetadataEndpoint,
    declared: &str,
) -> ControlFlow<ProviderMetadataRejection, Url> {
    match declared.parse::<HttpsUrl>() {
        Ok(url) => ControlFlow::Continue(Url::from(url)),
        Err(source) => ControlFlow::Break(ProviderMetadataRejection::Endpoint { endpoint, source }),
    }
}

fn advertised_endpoint(
    endpoint: MetadataEndpoint,
    declared: Option<String>,
) -> ControlFlow<ProviderMetadataRejection, AdvertisedEndpoint> {
    match declared {
        Some(declared) => {
            https_endpoint(endpoint, &declared).map_continue(AdvertisedEndpoint::Advertised)
        }
        None => ControlFlow::Continue(AdvertisedEndpoint::Unadvertised),
    }
}

fn authorization_response_issuer(advertised: bool) -> AuthorizationResponseIssuer {
    if advertised {
        AuthorizationResponseIssuer::Advertised
    } else {
        AuthorizationResponseIssuer::Unadvertised
    }
}

pub struct ProviderMetadata {
    pub authorization_endpoint: AdvertisedEndpoint,
    pub authorization_response_issuer: AuthorizationResponseIssuer,
    pub introspection_endpoint: AdvertisedEndpoint,
    pub jwks_uri: Url,
    pub token_endpoint: AdvertisedEndpoint,
    pub userinfo_endpoint: AdvertisedEndpoint,
}

impl ProviderMetadata {
    fn from_document(
        ProviderMetadataDocument {
            authorization_endpoint,
            authorization_response_iss_parameter_supported,
            introspection_endpoint,
            issuer: declared_issuer,
            jwks_uri,
            token_endpoint,
            userinfo_endpoint,
            ..
        }: ProviderMetadataDocument,
        issuer: &'static str,
    ) -> ControlFlow<ProviderMetadataRejection, Self> {
        if declared_issuer != issuer {
            return ControlFlow::Break(ProviderMetadataRejection::IssuerMismatch {
                expected: issuer,
                found: declared_issuer,
            });
        }

        ControlFlow::Continue(Self {
            authorization_endpoint: advertised_endpoint(
                MetadataEndpoint::AuthorizationEndpoint,
                authorization_endpoint,
            )?,
            authorization_response_issuer: authorization_response_issuer(
                authorization_response_iss_parameter_supported,
            ),
            introspection_endpoint: advertised_endpoint(
                MetadataEndpoint::IntrospectionEndpoint,
                introspection_endpoint,
            )?,
            jwks_uri: https_endpoint(MetadataEndpoint::JwksUri, &jwks_uri)?,
            token_endpoint: advertised_endpoint(MetadataEndpoint::TokenEndpoint, token_endpoint)?,
            userinfo_endpoint: advertised_endpoint(
                MetadataEndpoint::UserinfoEndpoint,
                userinfo_endpoint,
            )?,
        })
    }

    #[must_use]
    pub fn parse(document: &[u8], issuer: &'static str) -> ProviderMetadataParsing {
        match serde_json::from_slice(document) {
            Ok(document) => match Self::from_document(document, issuer) {
                ControlFlow::Continue(metadata) => {
                    ProviderMetadataParsing::Accepted(Arc::new(metadata))
                }
                ControlFlow::Break(rejection) => ProviderMetadataParsing::Rejected(rejection),
            },
            Err(source) => {
                ProviderMetadataParsing::Rejected(ProviderMetadataRejection::Malformed { source })
            }
        }
    }
}
