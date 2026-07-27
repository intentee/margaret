use std::path::Path;

use proc_macro2::TokenStream;
use quote::format_ident;

use margaret_container::container_bindings::ContainerBindings;
use margaret_container_tests::bindings_for_fixture::bindings_for_fixture;

fn bindings(fixture: &str) -> ContainerBindings {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    bindings_for_fixture("crate", &directory)
}

fn collapsed(tokens: TokenStream) -> String {
    tokens.to_string().split_whitespace().collect()
}

#[test]
fn reads_a_completed_dependency_without_async_or_failure_handling() {
    let bindings = bindings("fallible_constructor");
    let invocation = bindings.accessor_invocation(&format_ident!("container"), "loader");

    assert_eq!(collapsed(invocation), "container.loader()");
}

#[test]
fn invokes_the_selected_root_construction_function() {
    let bindings = bindings("fallible_constructor");
    let invocation = bindings.construction_invocation("loader", &[]);

    assert_eq!(
        collapsed(invocation),
        "super::container::build::construct_loader()"
    );
}
