use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
use margaret_oidc_discovery::provider_metadata_parsing::ProviderMetadataParsing;
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

fn advertised(url: &str) -> AdvertisedEndpoint {
    AdvertisedEndpoint::Advertised(url.parse().expect("the endpoint is a url"))
}

#[test]
fn describes_the_own_provider_by_its_endpoints() {
    assert!(matches!(
        ProviderMetadata::of_endpoints(&OWN_ENDPOINTS),
        ProviderMetadataParsing::Accepted(metadata)
            if metadata.authorization_endpoint == advertised("https://issuer.example/authorize")
                && metadata.authorization_response_issuer == AuthorizationResponseIssuer::Advertised
                && metadata.introspection_endpoint == advertised("https://issuer.example/introspect")
                && metadata.jwks_uri.as_str() == OWN_ENDPOINTS.jwks
                && metadata.token_endpoint == advertised(OWN_ENDPOINTS.token)
                && metadata.userinfo_endpoint == advertised("https://issuer.example/userinfo")
    ));
}

#[test]
fn leaves_the_endpoints_the_own_provider_does_not_serve_unadvertised() {
    assert!(matches!(
        ProviderMetadata::of_endpoints(&ProviderEndpoints {
            authorization: ServedEndpoint::Unserved,
            introspection: ServedEndpoint::Unserved,
            userinfo: ServedEndpoint::Unserved,
            ..OWN_ENDPOINTS
        }),
        ProviderMetadataParsing::Accepted(metadata)
            if metadata.authorization_endpoint == AdvertisedEndpoint::Unadvertised
                && metadata.introspection_endpoint == AdvertisedEndpoint::Unadvertised
                && metadata.userinfo_endpoint == AdvertisedEndpoint::Unadvertised
    ));
}

#[test]
fn rejects_own_endpoints_that_are_not_https_urls() {
    let plaintext = "http://issuer.example/endpoint";
    let rejected = [
        ProviderEndpoints {
            authorization: ServedEndpoint::Served(plaintext),
            ..OWN_ENDPOINTS
        },
        ProviderEndpoints {
            introspection: ServedEndpoint::Served(plaintext),
            ..OWN_ENDPOINTS
        },
        ProviderEndpoints {
            jwks: plaintext,
            ..OWN_ENDPOINTS
        },
        ProviderEndpoints {
            token: plaintext,
            ..OWN_ENDPOINTS
        },
        ProviderEndpoints {
            userinfo: ServedEndpoint::Served(plaintext),
            ..OWN_ENDPOINTS
        },
    ]
    .iter()
    .map(
        |endpoints| match ProviderMetadata::of_endpoints(endpoints) {
            ProviderMetadataParsing::Accepted(_) => "accepted".to_string(),
            ProviderMetadataParsing::Rejected(rejection) => rejection.to_string(),
        },
    )
    .collect::<Vec<String>>();

    assert_eq!(
        rejected,
        [
            "authorization_endpoint",
            "introspection_endpoint",
            "jwks_uri",
            "token_endpoint",
            "userinfo_endpoint"
        ]
        .map(|member| format!(
            "the provider {member} is rejected: the url uses the 'http' scheme instead of https"
        ))
    );
}
