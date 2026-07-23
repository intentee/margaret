use std::path::Path;

use syn::parse_quote;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn named(name: &str) -> ConsoleArgument {
    ConsoleArgument::Named {
        name: name.to_string(),
        required: true,
        value_type: parse_quote!(String),
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

    let union = bindings.console_union(&[named("path"), named("alpha"), named("path")]);
    let names: Vec<&str> = union.iter().map(ConsoleArgument::name).collect();
    let slots: Vec<usize> = union
        .iter()
        .map(|argument| bindings.console_slot(argument.name()))
        .collect();

    assert_eq!(union.len(), 2);
    assert!(names.contains(&"path"));
    assert!(names.contains(&"alpha"));
    assert!(slots[0] < slots[1]);
}

#[test]
fn renders_the_console_argument_threading_tokens() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let bindings = bindings_for_fixture("crate", &directory);
    let slot = bindings.console_slot("path");
    let arguments = [named("path")];

    let parameters = bindings.console_parameters(&arguments);
    let borrows = bindings.console_borrows(&arguments);
    let forwards = bindings.console_forwards(&arguments);
    let threads = bindings.console_threads(&arguments);

    assert_eq!(
        collapsed(&parameters[0]),
        format!("console_argument_{slot}:&String,")
    );
    assert_eq!(collapsed(&borrows[0]), format!("&console_argument_{slot},"));
    assert_eq!(collapsed(&forwards[0]), format!("console_argument_{slot},"));
    assert_eq!(
        collapsed(&threads[0]),
        format!("console_argument_{slot}.clone()")
    );
}
