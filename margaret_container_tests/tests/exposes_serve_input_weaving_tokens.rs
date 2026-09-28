use std::fmt::Display;
use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;
use margaret_input_weaving::input_value::InputValue;
use margaret_input_weaving::weaving_kind::WeavingKind;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_key::ServeInputKey;

fn console_key(name: &str) -> ServeInputKey {
    ServeInputKey::ConsoleArgument {
        name: name.to_string(),
    }
}

fn named(name: &str) -> ServeInput {
    ServeInput::ConsoleArgument(ConsoleArgument::Named {
        name: name.to_string(),
        value: InputValue {
            required: true,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
            weaving: WeavingKind::BorrowedStr,
        },
    })
}

fn copy_named(name: &str) -> ServeInput {
    ServeInput::ConsoleArgument(ConsoleArgument::Named {
        name: name.to_string(),
        value: InputValue {
            required: true,
            value_type: CanonicalPath::new(vec!["u16".to_string()]),
            weaving: WeavingKind::Copy,
        },
    })
}

fn collapsed<Token: Display>(token: &Token) -> String {
    token.to_string().split_whitespace().collect()
}

#[test]
fn unifies_and_orders_a_serve_input_union_by_slot() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let union = bindings
        .serve_input_union(&[named("path"), named("alpha"), named("path")])
        .expect("the planned serve inputs have assigned slots");
    let names: Vec<&str> = union.iter().map(ServeInput::name).collect();
    let slots: Vec<usize> = union
        .iter()
        .map(|input| bindings.serve_input_slot(&input.slot_key()))
        .collect::<Result<Vec<_>, _>>()
        .expect("every unified serve input has an assigned slot");

    assert_eq!(union.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));
    assert!(slots[0] < slots[1]);
}

#[test]
fn clones_a_non_copy_serve_input_that_is_not_its_final_use() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let slot = bindings
        .serve_input_slot(&console_key("path"))
        .expect("the path argument has a slot");
    let weaves = bindings
        .serve_input_weaves_owned(&[named("path")])
        .expect("the planned serve input weaves");

    assert_eq!(collapsed(&weaves[0]), format!("serve_input_{slot}.clone()"));
}

#[test]
fn moves_a_copy_serve_input_even_when_it_is_not_its_final_use() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_copy_fanout");
    let bindings = bindings_for_fixture("crate", &directory);

    let slot = bindings
        .serve_input_slot(&console_key("retries"))
        .expect("the retries argument has a slot");
    let weaves = bindings
        .serve_input_weaves_owned(&[copy_named("retries")])
        .expect("the planned serve input weaves");

    assert_eq!(collapsed(&weaves[0]), format!("serve_input_{slot}"));
}
