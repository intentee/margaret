use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[test]
fn rejects_metadata_of_another_issuer() {
    let issuer = "https://server.example.com";

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://attacker.example","jwks_uri":"https://attacker.example/jwks"}"#, issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::IssuerMismatch { found, .. }) if found == "https://attacker.example"
    ));
}
