use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

fn fixture(name: &str) -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);

    generate_container_source("crate", &directory)
        .expect("the fixture generates a container")
        .source()
        .split_whitespace()
        .collect()
}

#[test]
fn constructs_a_singleton_returning_self_directly_since_its_constructor_cannot_fail() {
    let source = fixture("infallible_constructor");

    assert!(source.contains("letclock=::std::sync::Arc::new(crate::Clock::new());"));
    assert!(!source.contains("construct_singleton"));
}

#[test]
fn constructs_a_singleton_naming_its_own_struct_as_the_return_type_directly() {
    let source = fixture("infallible_constructor");

    assert!(source.contains(
        "letgreeter=::std::sync::Arc::new(crate::Greeter::new(::std::sync::Arc::clone(&clock)),);"
    ));
}

#[test]
fn renders_a_root_builder_returning_its_root_when_nothing_in_its_flow_can_fail() {
    let source = fixture("infallible_constructor");

    assert!(source.contains("pubfnconstruct_greeter()->::std::sync::Arc<crate::Greeter>{"));
    assert!(!source.contains("ConstructionError"));
}

#[test]
fn returns_the_root_of_an_infallible_builder_as_its_tail_expression_instead_of_a_binding() {
    let source = fixture("infallible_constructor");

    assert!(source.contains(
        "pubfnconstruct_greeter()->::std::sync::Arc<crate::Greeter>{letclock=::std::sync::Arc::new(crate::Clock::new());::std::sync::Arc::new(crate::Greeter::new(::std::sync::Arc::clone(&clock)))}"
    ));
}

#[test]
fn renders_serve_returning_the_container_when_nothing_it_constructs_can_fail() {
    let source = fixture("infallible_constructor");

    assert!(source.contains("pubfnserve()->super::super::Container{"));
}

#[test]
fn keeps_an_infallible_singleton_root_fallible_when_a_dependency_can_fail() {
    let source = fixture("infallible_dependent_of_fallible");

    assert!(source.contains(
        "pubfnconstruct_reporter()->::std::result::Result<::std::sync::Arc<crate::Reporter>,margaret::framework::construction_error::ConstructionError,>{"
    ));
    assert!(source.contains(
        "letreporter=::std::sync::Arc::new(crate::Reporter::new(::std::sync::Arc::clone(&loader)),);"
    ));
}

#[test]
fn marks_an_infallible_synchronous_builder_must_use_since_its_value_is_the_construction() {
    let source = fixture("infallible_constructor");

    assert!(source.contains(
        "#[must_use]#[allow(clippy::too_many_lines,reason=\"onestatementperdeclareditem;thegeneratedlististheunitofmeaning\")]pubfnserve()->super::super::Container{"
    ));
}

#[test]
fn leaves_must_use_to_the_result_of_a_fallible_builder() {
    assert!(!fixture("infallible_dependent_of_fallible").contains("#[must_use]"));
}

#[test]
fn awaits_an_infallible_asynchronous_constructor_without_must_use_since_the_future_carries_it() {
    let source = fixture("infallible_async_constructor");

    assert!(source.contains("letpool=::std::sync::Arc::new(crate::Pool::new().await);"));
    assert!(source.contains("pubasyncfnserve()->super::super::Container{"));
    assert!(!source.contains("#[must_use]"));
}
