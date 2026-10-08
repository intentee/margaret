use margaret_https_url::https_url_rejection::HttpsUrlRejection;
use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[test]
fn rejects_a_plaintext_jwks_uri() {
    let issuer = "https://server.example.com";

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://server.example.com","jwks_uri":"http://server.example.com/jwks.json"}"#, issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::Endpoint { endpoint: MetadataEndpoint::JwksUri, rejection: HttpsUrlRejection::NotHttps { scheme } }) if scheme == "http"
    ));
}
