use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn wires_an_endpoint_provider_by_tag() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/endpoint_provider");
    let generated = generate_container_source("crate", &directory)
        .expect("the endpoint provider fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("crate::Poller::new(self.internal_endpoint().await)"));
}
