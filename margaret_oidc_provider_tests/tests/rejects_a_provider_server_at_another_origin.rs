use margaret_http::server_origin::ServerOrigin;
use margaret_oidc_provider_tests::provider_endpoints_of_the_fixture::provider_endpoints_of_the_fixture;

#[test]
fn rejects_a_provider_server_at_another_origin() {
    assert_eq!(
        provider_endpoints_of_the_fixture()
            .served_by(
                &"https://localhost:8443"
                    .parse::<ServerOrigin>()
                    .expect("the server url is an origin")
            )
            .expect_err("the provider server is at another origin")
            .to_string(),
        "the issuer at 'https://localhost' is not served by the provider server at 'https://localhost:8443'"
    );
}
