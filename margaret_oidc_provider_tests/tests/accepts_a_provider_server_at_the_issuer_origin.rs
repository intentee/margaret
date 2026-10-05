use margaret_http::server_origin::ServerOrigin;
use margaret_oidc_provider_tests::provider_endpoints_of_the_fixture::provider_endpoints_of_the_fixture;

#[test]
fn accepts_a_provider_server_at_the_issuer_origin() {
    provider_endpoints_of_the_fixture()
        .served_by(
            &"https://localhost:443/"
                .parse::<ServerOrigin>()
                .expect("the server url is an origin"),
        )
        .expect("the provider server serves the issuer origin");
}
