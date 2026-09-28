use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn rejects_an_issuer_that_differs_by_a_terminating_slash() {
    let issuer = "https://token.actions.githubusercontent.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://token.actions.githubusercontent.com/","jwks_uri":"https://token.actions.githubusercontent.com/.well-known/jwks"}"#, &issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::IssuerMismatch { .. })
    ));
}
