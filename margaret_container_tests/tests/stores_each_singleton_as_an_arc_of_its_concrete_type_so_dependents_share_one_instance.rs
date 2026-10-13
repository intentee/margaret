use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn stores_each_singleton_as_an_arc_of_its_concrete_type_so_dependents_share_one_instance() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("crate", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("::std::sync::Arc<crate::Config>"));
    assert!(source.contains("::std::sync::Arc<crate::EnglishGreeter>"));
}
