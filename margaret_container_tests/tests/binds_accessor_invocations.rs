use std::path::Path;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_container::accessor_failure::AccessorFailure;
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
fn reports_a_container_that_constructs_singletons() {
    assert!(bindings("services").has_accessors());
}

#[test]
fn propagates_an_accessor_with_the_question_mark_operator() {
    let bindings = bindings("fallible_constructor");
    let invocation = bindings.accessor_invocation(
        &format_ident!("container"),
        "loader",
        &[],
        &AccessorFailure::Propagate,
    );

    assert_eq!(collapsed(invocation), "container.loader().await?");
}

#[test]
fn reports_an_accessor_into_a_caller_supplied_outcome() {
    let bindings = bindings("fallible_constructor");
    let invocation = bindings.accessor_invocation(
        &format_ident!("container"),
        "loader",
        &[],
        &AccessorFailure::Report(quote! { return outcome }),
    );

    assert_eq!(
        collapsed(invocation),
        "(matchcontainer.loader().await{Ok(value)=>value,Err(error)=>returnoutcome,})"
    );
}

#[test]
fn propagates_even_an_infallible_constructor_accessor() {
    let bindings = bindings("services");
    let invocation = bindings.accessor_invocation(
        &format_ident!("container"),
        "pulse",
        &[],
        &AccessorFailure::Propagate,
    );

    assert_eq!(collapsed(invocation), "container.pulse().await?");
}
