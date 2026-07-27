use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn constructs_services_and_tickers_eagerly() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/services");
    let generated = generate_container_source("crate", &directory)
        .expect("the services fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("pubasyncfnconstruct_pulse("));
    assert!(source.contains("pubasyncfnconstruct_sweeper("));
    assert!(source.contains("pubasyncfnserve("));
    assert!(!source.contains("construct_once"));
}
