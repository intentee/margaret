use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn recognizes_an_advertised_authorization_response_issuer() {
    let issuer = "https://server.example.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");
    let ProviderMetadataParsing::Accepted(metadata) = ProviderMetadata::parse(
        br#"{"issuer":"https://server.example.com","jwks_uri":"https://server.example.com/jwks.json","authorization_response_iss_parameter_supported":true}"#,
        &issuer,
    ) else {
        panic!("the metadata is accepted");
    };

    assert_eq!(
        metadata.authorization_response_issuer,
        AuthorizationResponseIssuer::Advertised
    );
}
