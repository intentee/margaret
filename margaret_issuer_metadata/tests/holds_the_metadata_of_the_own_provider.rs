use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_metadata::issuer_metadata_error::IssuerMetadataError;
use margaret_issuer_metadata::metadata_holding::MetadataHolding;
use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::served_endpoint::ServedEndpoint;

const OWN_ENDPOINTS: ProviderEndpoints = ProviderEndpoints {
    authorization: ServedEndpoint::Served("https://issuer.example/authorize"),
    introspection: ServedEndpoint::Served("https://issuer.example/introspect"),
    issuer_origin: "https://issuer.example",
    jwks: "https://issuer.example/jwks.json",
    revocation: ServedEndpoint::Served("https://issuer.example/revoke"),
    token: "https://issuer.example/token",
    userinfo: ServedEndpoint::Served("https://issuer.example/userinfo"),
};

#[test]
fn holds_the_metadata_of_the_own_provider_from_the_start() {
    assert!(matches!(
        IssuerMetadata::of_provider(OWN_ENDPOINTS).map(|metadata| metadata.holding()),
        Ok(MetadataHolding::Held(metadata)) if metadata.jwks_uri.as_str() == OWN_ENDPOINTS.jwks
    ));
}

#[test]
fn reports_own_endpoints_that_do_not_describe_metadata() {
    assert!(matches!(
        IssuerMetadata::of_provider(ProviderEndpoints {
            token: "http://issuer.example/token",
            ..OWN_ENDPOINTS
        }),
        Err(IssuerMetadataError::OwnProviderEndpoints { .. })
    ));
}
