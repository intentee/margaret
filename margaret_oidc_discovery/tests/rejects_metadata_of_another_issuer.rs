use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn rejects_metadata_of_another_issuer() {
    let issuer = "https://server.example.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://attacker.example","jwks_uri":"https://attacker.example/jwks"}"#, &issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::IssuerMismatch { found, .. }) if found == "https://attacker.example"
    ));
}
