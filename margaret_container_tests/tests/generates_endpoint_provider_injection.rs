use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

fn endpoints_container() -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoints");

    generate_container_source("endpoints", &directory)
        .expect("the endpoint fixture renders")
        .source()
        .split_whitespace()
        .collect()
}

#[test]
fn binds_an_endpoint_provider_as_the_provides_endpoint_interface() {
    assert!(
        endpoints_container()
            .contains("std::sync::Arc<dynmargaret_endpoint::provides_endpoint::ProvidesEndpoint>")
    );
}

#[test]
fn injects_the_endpoint_providers_own_dependencies_and_console_argument() {
    assert!(
        endpoints_container().contains(
            "endpoints::JwksEndpoint::new(self.dns_resolver().await,console_argument_0,)"
        )
    );
}

#[test]
fn injects_the_endpoint_provider_into_a_consumer_by_tag() {
    assert!(
        endpoints_container()
            .contains("endpoints::JwksClient::new(self.jwks_endpoint(console_argument_0).await,)")
    );
}
