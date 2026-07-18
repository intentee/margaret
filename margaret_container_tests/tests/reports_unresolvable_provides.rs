use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_unresolvable_provides() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provides_unresolvable");
    let error = generate_container_source("provides_unresolvable", &directory)
        .expect_err("a provides interface matching no trait must be rejected");

    assert!(matches!(error, ContainerError::ProvidesUnresolvable { .. }));
}
