use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn path(name: &str) -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
}

#[test]
fn unifies_serve_inputs_across_roots_in_slot_order() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let serve = bindings
        .serve_inputs(&[path("Config"), path("Service")])
        .expect("the service roots have complete serve input plans");
    let names: Vec<&str> = serve.iter().map(|slotted| slotted.input.name()).collect();

    assert_eq!(serve.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));
    assert!(serve[0].slot < serve[1].slot);
    assert_eq!(
        bindings
            .provider_serve_inputs(&path("Config"))
            .expect("Config has planned serve inputs")
            .len(),
        1
    );
}
