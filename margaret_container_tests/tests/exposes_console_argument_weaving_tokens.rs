use std::path::Path;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::serve_input_key::ServeInputKey;
use margaret_console_argument_codegen::weaving_kind::WeavingKind;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn console_key(name: &str) -> ServeInputKey {
    ServeInputKey::ConsoleArgument {
        name: name.to_string(),
    }
}

fn named(name: &str) -> ConsoleArgument {
    ConsoleArgument::Named {
        name: name.to_string(),
        required: true,
        weaving: WeavingKind::BorrowedStr,
        value_type: CanonicalPath::new(vec![
            "std".to_string(),
            "string".to_string(),
            "String".to_string(),
        ]),
    }
}

fn copy_named(name: &str) -> ConsoleArgument {
    ConsoleArgument::Named {
        name: name.to_string(),
        required: true,
        weaving: WeavingKind::Copy,
        value_type: CanonicalPath::new(vec!["u16".to_string()]),
    }
}

fn collapsed<Token: std::fmt::Display>(token: &Token) -> String {
    token.to_string().split_whitespace().collect()
}

#[test]
fn unifies_and_orders_a_console_argument_union_by_slot() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let bindings = bindings_for_fixture("crate", &directory);

    let union = bindings
        .console_union(&[named("path"), named("alpha"), named("path")])
        .expect("the planned console arguments have assigned slots");
    let names: Vec<&str> = union.iter().map(ConsoleArgument::name).collect();
    let slots: Vec<usize> = union
        .iter()
        .map(|argument| bindings.console_slot(&argument.slot_key()))
        .collect::<Result<Vec<_>, _>>()
        .expect("every unified console argument has an assigned slot");

    assert_eq!(union.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));
    assert!(slots[0] < slots[1]);
}

#[test]
fn renders_owned_console_argument_construction_tokens() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let bindings = bindings_for_fixture("crate", &directory);
    let slot = bindings
        .console_slot(&console_key("path"))
        .expect("the path argument has a slot");
    let arguments = [named("path")];

    let weaves = bindings
        .console_weaves_owned(&arguments)
        .expect("the path argument can be woven");

    assert_eq!(
        collapsed(&weaves[0]),
        format!("console_argument_{slot}.clone()")
    );
}

#[test]
fn renders_a_copy_input_without_a_clone() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_copy_fanout");
    let bindings = bindings_for_fixture("crate", &directory);
    let slot = bindings
        .console_slot(&console_key("retries"))
        .expect("the retries argument has a slot");
    let weaves = bindings
        .console_weaves_owned(&[copy_named("retries")])
        .expect("the copy argument can be woven");

    assert_eq!(collapsed(&weaves[0]), format!("console_argument_{slot}"));
}
