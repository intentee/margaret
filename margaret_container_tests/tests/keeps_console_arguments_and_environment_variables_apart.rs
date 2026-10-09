use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

#[test]
fn allocates_a_separate_slot_to_each_namespace() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/console_argument_and_environment_variable_same_name");
    let bindings = bindings_for_fixture("crate", &directory);

    let inputs = bindings
        .provider_serve_inputs(&CanonicalPath::new(vec![
            "crate".to_string(),
            "Config".to_string(),
        ]))
        .expect("Config has planned serve inputs");

    assert_eq!(inputs.len(), 2);
    assert_ne!(inputs[0].slot, inputs[1].slot);
}
