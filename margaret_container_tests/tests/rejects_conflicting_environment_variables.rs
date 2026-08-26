use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

fn rejection_message(fixture: &str) -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source("crate", &directory)
        .expect_err("the fixture is rejected")
        .to_string()
}

#[test]
fn rejects_one_variable_declared_with_two_value_types() {
    assert!(
        rejection_message("environment_variable_type_conflict")
            .contains("a shared serve input must be declared identically everywhere")
    );
}

#[test]
fn rejects_a_parameter_that_declares_two_serve_inputs() {
    assert!(
        rejection_message("ambiguous_serve_input")
            .contains("a constructor parameter is fed by exactly one serve input")
    );
}
