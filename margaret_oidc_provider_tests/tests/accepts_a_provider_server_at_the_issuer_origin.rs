use crate::provider_endpoints_of_the_fixture::provider_endpoints_of_the_fixture;

#[test]
fn accepts_a_provider_server_at_the_issuer_origin() {
    provider_endpoints_of_the_fixture()
        .served_by("https://localhost:443/")
        .expect("the provider server serves the issuer origin");
}
