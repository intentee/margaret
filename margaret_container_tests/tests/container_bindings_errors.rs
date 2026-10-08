use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn bindings() -> ContainerBindings {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");

    bindings_for_fixture("crate", &directory)
}

fn missing_path() -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
}

#[test]
fn reports_a_serve_input_request_for_an_unplanned_component() {
    let error = bindings()
        .provider_serve_inputs(&missing_path())
        .expect_err("an unplanned component has no planned serve inputs");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_serve_root() {
    let error = bindings()
        .serve_inputs(&[missing_path()])
        .expect_err("an unplanned root cannot participate in serve construction");

    assert!(error.to_string().contains("crate::Missing"));
}
