use std::sync::Arc;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

fn metadata(jwks_uri: &str) -> Arc<ProviderMetadata> {
    let issuer = "https://issuer.example"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");
    let ProviderMetadataParsing::Accepted(metadata) = ProviderMetadata::parse(
        format!(r#"{{"issuer":"https://issuer.example","jwks_uri":"{jwks_uri}"}}"#).as_bytes(),
        &issuer,
    ) else {
        panic!("the fixture metadata is accepted");
    };

    metadata
}

#[test]
fn holds_the_latest_metadata() {
    let issuer_metadata = IssuerMetadata::awaiting();

    issuer_metadata.hold(metadata("https://issuer.example/first"));
    issuer_metadata.hold(metadata("https://issuer.example/second"));

    let MetadataHolding::Held(held) = issuer_metadata.holding() else {
        panic!("the metadata is held");
    };

    assert_eq!(held.jwks_uri.as_str(), "https://issuer.example/second");
}
