use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn generates_async_accessors() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/async_pool");
    let generated = generate_container_source("crate", &directory)
        .expect("the async fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains(
        "pool:margaret::framework::container_error::construction_slot::ConstructionSlot<crate::Pool,>"
    ));
    assert!(source.contains(
        "config:margaret::framework::container_error::construction_slot::ConstructionSlot<crate::Config,>"
    ));
    assert!(source.contains("pubasyncfnpool"));
    assert!(source.contains("crate::Pool::new(self.config().await?).await"));
    assert!(source.contains("crate::Config::new()"));
    assert!(!source.contains("crate::Config::new().await"));
}
