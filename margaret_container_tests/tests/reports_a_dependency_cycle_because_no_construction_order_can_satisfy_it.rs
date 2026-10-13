use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reports_a_dependency_cycle_because_no_construction_order_can_satisfy_it() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cycle");
    let error = generate_container_source("cycle", &directory)
        .expect_err("a cycle in the dependency graph must be rejected");

    assert!(matches!(error, ContainerError::DependencyCycle { .. }));
}
