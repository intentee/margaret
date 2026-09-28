use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

const DISCOVERY_SPECIFICATION_EXAMPLE_METADATA: &[u8] =
    include_bytes!("fixtures/openid_connect_discovery_example.json");

#[test]
fn accepts_the_discovery_specification_example_metadata() {
    let issuer = "https://server.example.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    let ProviderMetadataParsing::Accepted(metadata) =
        ProviderMetadata::parse(DISCOVERY_SPECIFICATION_EXAMPLE_METADATA, &issuer)
    else {
        panic!("the discovery specification example metadata is accepted");
    };

    assert_eq!(
        metadata.jwks_uri().as_str(),
        "https://server.example.com/jwks.json"
    );
}
