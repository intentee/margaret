use margaret_oidc_provider::provider_error::ProviderError;

use crate::provider_endpoints_of_the_fixture::provider_endpoints_of_the_fixture;

#[test]
fn rejects_a_provider_server_url_that_is_not_a_url() {
    let rejection = provider_endpoints_of_the_fixture()
        .served_by("localhost")
        .expect_err("the provider server url is not a url");

    assert!(
        matches!(&rejection, ProviderError::ServerUrlMalformed { server_url, .. } if server_url == "localhost")
    );
    assert!(
        rejection
            .to_string()
            .starts_with("the url 'localhost' of the provider server is not a url: ")
    );
}
