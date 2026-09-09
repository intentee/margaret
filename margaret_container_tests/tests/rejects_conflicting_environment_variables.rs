use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

fn rejection(fixture: &str) -> ContainerError {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source("crate", &directory).expect_err("the fixture is rejected")
}

#[test]
fn rejects_one_variable_declared_with_two_value_types() {
    assert!(matches!(
        rejection("environment_variable_type_conflict"),
        ContainerError::ServeInput {
            source: ServeInputCodegenError::ConflictingServeInput { .. }
        }
    ));
}

#[test]
fn rejects_a_parameter_that_declares_two_serve_inputs() {
    assert!(matches!(
        rejection("ambiguous_serve_input"),
        ContainerError::ServeInput {
            source: ServeInputCodegenError::AmbiguousServeInput { .. }
        }
    ));
}
