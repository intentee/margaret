use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_key::ServeInputKey;

fn bindings() -> ContainerBindings {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");

    bindings_for_fixture("crate", &directory)
}

fn missing_flag() -> ServeInput {
    ServeInput::ConsoleArgument(ConsoleArgument::Flag {
        name: "missing".to_string(),
    })
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
fn reports_an_unknown_serve_input_while_ordering_inputs() {
    let error = bindings()
        .serve_input_union(&[missing_flag()])
        .expect_err("an unknown input has no canonical slot");

    assert!(error.to_string().contains("missing"));
}

#[test]
fn reports_an_unknown_serve_input_while_materializing_inputs() {
    let error = bindings()
        .serve_input_weaves_owned(&[missing_flag()])
        .expect_err("an unknown input cannot be materialized");

    assert!(error.to_string().contains("missing"));
}

#[test]
fn reports_an_unplanned_injected_dependency() {
    let error = bindings()
        .injected_serve_inputs(&InjectedDependency {
            concrete: missing_path(),
            field: "missing".to_string(),
        })
        .expect_err("an unplanned dependency has no serve inputs");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_serve_root() {
    let error = bindings()
        .serve_inputs(&[missing_path()], &[])
        .expect_err("an unplanned root cannot participate in serve construction");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_a_serve_input_slot_that_the_plan_did_not_allocate() {
    let error = bindings()
        .serve_input_slot(&ServeInputKey::ConsoleArgument {
            name: "missing".to_string(),
        })
        .expect_err("an unknown input has no canonical slot");

    assert!(error.to_string().contains("missing"));
}
