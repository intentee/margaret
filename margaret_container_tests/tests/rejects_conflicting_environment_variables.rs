use std::mem::discriminant;
use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_error::ContainerError;
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

#[test]
fn rejects_one_variable_declared_with_two_value_types() {
    let error = rejection("environment_variable_type_conflict");

    assert_eq!(
        discriminant(
            serve_input_rejection(&error).expect("the fixture is rejected by the serve inputs")
        ),
        discriminant(&ServeInputCodegenError::ConflictingServeInput {
            first_owner: "any".to_string(),
            name: "any".to_string(),
            owner: "any".to_string(),
        })
    );
}

#[test]
fn rejects_a_parameter_that_declares_two_serve_inputs() {
    let error = rejection("ambiguous_serve_input");

    assert_eq!(
        discriminant(
            serve_input_rejection(&error).expect("the fixture is rejected by the serve inputs")
        ),
        discriminant(&ServeInputCodegenError::AmbiguousServeInput {
            first: "first",
            second: "second",
            site: ConstructorParameter {
                owner: CanonicalPath::new(vec!["crate".to_string(), "Any".to_string()]),
                parameter: "any".to_string(),
            },
        })
    );
}
