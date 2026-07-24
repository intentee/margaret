use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn constructs_services_and_tickers_lazily() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/services");
    let generated = generate_container_source("crate", &directory)
        .expect("the services fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("pubasyncfnpulse(&self)->std::sync::Arc<crate::Pulse>"));
    assert!(source.contains("pubasyncfnsweeper(&self)->std::sync::Arc<crate::Sweeper>"));
}
