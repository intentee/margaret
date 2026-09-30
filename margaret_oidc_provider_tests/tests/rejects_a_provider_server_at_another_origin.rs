use crate::provider_endpoints_of_the_fixture::provider_endpoints_of_the_fixture;

#[test]
fn rejects_a_provider_server_at_another_origin() {
    assert_eq!(
        provider_endpoints_of_the_fixture()
            .served_by("https://localhost:8443")
            .expect_err("the provider server is at another origin")
            .to_string(),
        "the issuer at 'https://localhost' is not served by the provider server at 'https://localhost:8443'"
    );
}
