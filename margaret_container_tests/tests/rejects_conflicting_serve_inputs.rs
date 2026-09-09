use std::path::Path;

use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
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
fn rejects_a_positional_colliding_with_a_named_console_argument() {
    assert!(matches!(
        rejection("serve_input_collision"),
        ContainerError::ServeInput {
            source: ServeInputCodegenError::ConflictingServeInput { .. }
        }
    ));
}

#[test]
fn rejects_a_positional_outside_a_console_command() {
    assert!(matches!(
        rejection("positional_outside_command"),
        ContainerError::ServeInput {
            source: ServeInputCodegenError::ConsoleArgument {
                source: ConsoleArgumentCodegenError::PositionalOutsideCommand { .. }
            }
        }
    ));
}

#[test]
fn rejects_a_collision_reached_through_a_dependency() {
    assert!(matches!(
        rejection("serve_input_dependency_collision"),
        ContainerError::ServeInput {
            source: ServeInputCodegenError::ConflictingServeInput { .. }
        }
    ));
}
