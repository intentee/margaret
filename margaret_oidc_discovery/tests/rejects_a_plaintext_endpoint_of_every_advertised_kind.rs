use serde_json::json;

use margaret_oidc_discovery::metadata_endpoint::MetadataEndpoint;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

fn rejects_a_plaintext(endpoint: MetadataEndpoint) -> bool {
    let issuer = "https://server.example.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");
    let mut document = json!({
        "issuer": "https://server.example.com",
        "jwks_uri": "https://server.example.com/jwks.json",
    });

    document
        .as_object_mut()
        .expect("the document is an object")
        .insert(
            endpoint.member_name().to_string(),
            json!("http://server.example.com/endpoint"),
        );

    matches!(
        ProviderMetadata::parse(document.to_string().as_bytes(), &issuer),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::EndpointNotHttps { endpoint: rejected, .. })
            if rejected == endpoint
    )
}

#[test]
fn rejects_a_plaintext_endpoint_of_every_advertised_kind() {
    assert!(rejects_a_plaintext(MetadataEndpoint::AuthorizationEndpoint));
    assert!(rejects_a_plaintext(MetadataEndpoint::IntrospectionEndpoint));
    assert!(rejects_a_plaintext(MetadataEndpoint::TokenEndpoint));
    assert!(rejects_a_plaintext(MetadataEndpoint::UserinfoEndpoint));
}
