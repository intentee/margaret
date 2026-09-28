use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[test]
fn rejects_a_jwks_uri_that_is_not_a_url() {
    let issuer = "https://token.actions.githubusercontent.com"
        .parse::<IssuerIdentifier>()
        .expect("the issuer is an https url");

    assert!(matches!(
        ProviderMetadata::parse(
            br#"{"issuer":"https://token.actions.githubusercontent.com","jwks_uri":"not a url"}"#,
            &issuer
        ),
        ProviderMetadataParsing::Rejected(ProviderMetadataRejection::JwksUriMalformed { .. })
    ));
}
