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
fn rejects_a_positional_colliding_with_a_named_console_argument() {
    assert!(
        rejection_message("serve_input_collision")
            .contains("a shared serve input must be declared identically everywhere")
    );
}

#[test]
fn rejects_a_positional_outside_a_console_command() {
    assert!(
        rejection_message("positional_outside_command")
            .contains("positional arguments are only allowed on console commands")
    );
}

#[test]
fn rejects_a_collision_reached_through_a_dependency() {
    assert!(
        rejection_message("serve_input_dependency_collision")
            .contains("a shared serve input must be declared identically everywhere")
    );
}
