use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container::generate_container_source::generate_container_source;

#[test]
fn reports_provider_dependency_cycle() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/provider_cycle");
    let error = generate_container_source("provider_cycle", &directory)
        .err()
        .expect("a provider whose dependencies form a cycle must be rejected");

    assert!(matches!(error, ContainerError::DependencyCycle { .. }));
}
