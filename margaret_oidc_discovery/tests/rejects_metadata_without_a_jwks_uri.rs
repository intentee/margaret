use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[test]
fn rejects_metadata_without_a_jwks_uri() {
    let issuer = "https://server.example.com";

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://server.example.com"}"#, issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::Malformed { .. })
    ));
}
