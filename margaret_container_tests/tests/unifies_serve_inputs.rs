use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_key::ServeInputKey;

fn path(name: &str) -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
}

fn console_key(name: &str) -> ServeInputKey {
    ServeInputKey::ConsoleArgument {
        name: name.to_string(),
    }
}

#[test]
fn unifies_serve_inputs_across_roots() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let serve = bindings
        .serve_inputs(&[path("Config"), path("Service")], &[])
        .expect("the service roots have complete serve input plans");
    let names: Vec<&str> = serve.iter().map(ServeInput::name).collect();

    assert_eq!(serve.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));

    assert_eq!(
        bindings
            .provider_serve_inputs(&path("Config"))
            .expect("Config has planned serve inputs")
            .inputs
            .len(),
        1
    );
    assert!(bindings.provider_serve_inputs(&path("Absent")).is_err());

    let path_slot = bindings
        .serve_input_slot(&console_key("path"))
        .expect("the path argument has a slot");
    let alpha_slot = bindings
        .serve_input_slot(&console_key("alpha"))
        .expect("the alpha argument has a slot");

    assert_ne!(path_slot, alpha_slot);
}
