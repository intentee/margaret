use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn rejects_a_plaintext_jwks_uri() {
    let issuer = "https://token.actions.githubusercontent.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert!(matches!(
        ProviderMetadata::parse(br#"{"issuer":"https://token.actions.githubusercontent.com","jwks_uri":"http://token.actions.githubusercontent.com/.well-known/jwks"}"#, &issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::JwksUriNotHttps { scheme }) if scheme == "http"
    ));
}
