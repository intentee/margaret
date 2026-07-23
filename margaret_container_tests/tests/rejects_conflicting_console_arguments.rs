use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn rejects_a_positional_colliding_with_a_named_console_argument() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_collision");
    let error = generate_container_source("crate", &directory)
        .expect_err("a positional/named collision within a command is rejected");

    assert!(
        error
            .to_string()
            .contains("declared both as a positional and as a named argument")
    );
}

#[test]
fn rejects_a_positional_console_argument_outside_a_command() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/positional_outside_command");
    let error = generate_container_source("crate", &directory)
        .expect_err("a positional argument outside a console command is rejected");

    assert!(
        error
            .to_string()
            .contains("positional arguments are only allowed on console commands")
    );
}

#[test]
fn rejects_a_collision_reached_through_a_dependency() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/console_argument_dependency_collision");
    let error = generate_container_source("crate", &directory)
        .expect_err("a collision inside a dependency is rejected");

    assert!(
        error
            .to_string()
            .contains("declared both as a positional and as a named argument")
    );
}

#[test]
fn rejects_a_collision_reached_through_a_collection_member() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/console_argument_collection_collision");
    let error = generate_container_source("crate", &directory)
        .expect_err("a collision inside a collection member is rejected");

    assert!(
        error
            .to_string()
            .contains("declared both as a positional and as a named argument")
    );
}
