use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

const GITHUB_ACTIONS_PROVIDER_METADATA: &[u8] =
    include_bytes!("fixtures/github_actions_openid_configuration.json");

#[test]
fn accepts_the_github_actions_provider_metadata() {
    let issuer = "https://token.actions.githubusercontent.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    let ProviderMetadataParsing::Accepted(metadata) =
        ProviderMetadata::parse(GITHUB_ACTIONS_PROVIDER_METADATA, &issuer)
    else {
        panic!("the github actions provider metadata is accepted");
    };

    assert_eq!(
        metadata.jwks_uri().as_str(),
        "https://token.actions.githubusercontent.com/.well-known/jwks"
    );
}
