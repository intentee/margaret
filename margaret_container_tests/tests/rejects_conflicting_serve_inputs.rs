use std::mem::discriminant;
use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_container::container_error::ContainerError;
use margaret_container_tests::console_argument_rejection::console_argument_rejection;
use margaret_container_tests::generate_container_source::generate_container_source;
use margaret_container_tests::serve_input_rejection::serve_input_rejection;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

fn rejection(fixture: &str) -> ContainerError {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source("crate", &directory).expect_err("the fixture is rejected")
}

fn any_text() -> String {
    "any".to_string()
}

fn any_site() -> ConstructorParameter {
    ConstructorParameter {
        owner: CanonicalPath::new(vec!["crate".to_string(), "Any".to_string()]),
        parameter: any_text(),
    }
}

#[test]
fn rejects_a_positional_colliding_with_a_named_console_argument() {
    let error = rejection("serve_input_collision");

    assert_eq!(
        discriminant(
            serve_input_rejection(&error).expect("the fixture is rejected by the serve inputs")
        ),
        discriminant(&ServeInputCodegenError::ConflictingServeInput {
            first_owner: any_text(),
            name: any_text(),
            owner: any_text(),
        })
    );
}

#[test]
fn rejects_a_positional_outside_a_console_command() {
    let error = rejection("positional_outside_command");
    let serve_input =
        serve_input_rejection(&error).expect("the fixture is rejected by the serve inputs");

    assert_eq!(
        discriminant(
            console_argument_rejection(serve_input)
                .expect("the fixture is rejected by the console argument")
        ),
        discriminant(&ConsoleArgumentCodegenError::PositionalOutsideCommand { site: any_site() })
    );
}

#[test]
fn rejects_a_collision_reached_through_a_dependency() {
    let error = rejection("serve_input_dependency_collision");

    assert_eq!(
        discriminant(
            serve_input_rejection(&error).expect("the fixture is rejected by the serve inputs")
        ),
        discriminant(&ServeInputCodegenError::ConflictingServeInput {
            first_owner: any_text(),
            name: any_text(),
            owner: any_text(),
        })
    );
}

#[test]
fn reads_no_serve_input_rejection_from_an_unrelated_container_error() {
    assert!(
        serve_input_rejection(&ContainerError::NotASingletonStruct { path: any_text() }).is_none()
    );
}

#[test]
fn reads_no_console_argument_rejection_from_an_unrelated_serve_input_error() {
    assert!(
        console_argument_rejection(&ServeInputCodegenError::ConflictingServeInput {
            first_owner: any_text(),
            name: any_text(),
            owner: any_text(),
        })
        .is_none()
    );
}
