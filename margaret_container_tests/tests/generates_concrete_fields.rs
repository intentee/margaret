use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn generates_concrete_fields() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("crate", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains(
        "config:margaret::framework::container_error::construction_slot::ConstructionSlot<crate::Config,>"
    ));
    assert!(source.contains(
        "english_greeter:margaret::framework::container_error::construction_slot::ConstructionSlot<crate::EnglishGreeter,>"
    ));
}
