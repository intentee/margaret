use url::Url;

use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

const DISCOVERY_SPECIFICATION_EXAMPLE_METADATA: &[u8] =
    include_bytes!("fixtures/openid_connect_discovery_example.json");

fn advertised(url: &str) -> AdvertisedEndpoint {
    AdvertisedEndpoint::Advertised(Url::parse(url).expect("the example endpoint is a url"))
}

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
        metadata.authorization_endpoint,
        advertised("https://server.example.com/connect/authorize")
    );
    assert_eq!(
        metadata.authorization_response_issuer,
        AuthorizationResponseIssuer::Unadvertised
    );
    assert_eq!(
        metadata.introspection_endpoint,
        AdvertisedEndpoint::Unadvertised
    );
    assert_eq!(
        metadata.jwks_uri.as_str(),
        "https://server.example.com/jwks.json"
    );
    assert_eq!(
        metadata.token_endpoint,
        advertised("https://server.example.com/connect/token")
    );
    assert_eq!(
        metadata.userinfo_endpoint,
        advertised("https://server.example.com/connect/userinfo")
    );
}
