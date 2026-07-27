use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn bindings() -> margaret_container::container_bindings::ContainerBindings {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");

    bindings_for_fixture("crate", &directory)
}

fn missing_path() -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
}

#[test]
fn reports_a_console_argument_request_for_an_unplanned_component() {
    let error = bindings()
        .console_arguments(&missing_path())
        .expect_err("an unplanned component has no console argument closure");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unknown_console_input_while_ordering_arguments() {
    let error = bindings()
        .console_union(&[ConsoleArgument::Flag {
            name: "missing".to_string(),
        }])
        .expect_err("an unknown input has no canonical slot");

    assert!(error.to_string().contains("missing"));
}

#[test]
fn reports_an_unknown_console_input_while_materializing_arguments() {
    let error = bindings()
        .console_weaves_owned(&[ConsoleArgument::Flag {
            name: "missing".to_string(),
        }])
        .expect_err("an unknown input cannot be materialized");

    assert!(error.to_string().contains("missing"));
}

#[test]
fn reports_an_unplanned_injected_dependency() {
    let error = bindings()
        .injected_console_arguments(&InjectedDependency {
            concrete: missing_path(),
            field: "missing".to_string(),
        })
        .expect_err("an unplanned dependency has no console arguments");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_serve_root() {
    let error = bindings()
        .serve_arguments(&[missing_path()], &[])
        .expect_err("an unplanned root cannot participate in serve construction");

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_a_console_slot_that_the_plan_did_not_allocate() {
    let error = bindings()
        .console_slot(&ServeInputKey::ConsoleArgument {
            name: "missing".to_string(),
        })
        .expect_err("an unknown input has no canonical slot");

    assert!(error.to_string().contains("missing"));
}
