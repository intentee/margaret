use margaret_https_url::https_url_error::HttpsUrlError;
use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[test]
fn rejects_a_jwks_uri_that_is_not_a_url() {
    let issuer = "https://server.example.com";

    assert!(matches!(
        ProviderMetadata::parse(
            br#"{"issuer":"https://server.example.com","jwks_uri":"not a url"}"#,
            issuer
        ),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::Endpoint {
            endpoint: MetadataEndpoint::JwksUri,
            source: HttpsUrlError::Malformed { .. },
        })
    ));
}
