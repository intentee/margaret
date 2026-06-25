use std::path::Path;

use margaret_container::generate_container_source;

#[test]
fn generates_fieldless_singletons() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fieldless");
    let generated = generate_container_source("fieldless", &directory)
        .expect("fieldless singletons generate without a #[constructor]");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("std::sync::Arc::new(crate::UnitMarker)"));
    assert!(source.contains("std::sync::Arc::new(crate::EmptyNamed{})"));
    assert!(source.contains("std::sync::Arc::new(crate::EmptyTuple())"));
}
