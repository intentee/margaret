use std::path::Path;

use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;
use margaret_serve_input_codegen::serve_input_key::ServeInputKey;

#[test]
fn allocates_a_separate_slot_to_each_namespace() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/console_argument_and_environment_variable_same_name");
    let bindings = bindings_for_fixture("crate", &directory);

    let argument_slot = bindings
        .serve_input_slot(&ServeInputKey::ConsoleArgument {
            name: "DATABASE_URL".to_string(),
        })
        .expect("the console argument has a slot");
    let variable_slot = bindings
        .serve_input_slot(&ServeInputKey::EnvironmentVariable {
            name: "DATABASE_URL".to_string(),
        })
        .expect("the environment variable has a slot");

    assert_ne!(argument_slot, variable_slot);
}
