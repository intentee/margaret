use margaret_codegen::generated_code::GeneratedCode;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn generated() -> GeneratedCode {
    generate_fixture("copy_console_arguments").expect("the fixture generates")
}

fn module(generated: &GeneratedCode, name: &str) -> String {
    generated_module_source(generated, name)
        .expect("the module is generated")
        .split_whitespace()
        .collect()
}

#[test]
fn reads_a_required_standard_library_copy_argument_without_cloning() {
    let run = module(&generated(), "run");

    assert!(run.contains(
        "margaret::framework::console::required_argument::required_argument::<::std::num::NonZeroU32,>(matches,\"max-connections\").and_then(|serve_input_1|"
    ));
    assert!(!run.contains(".clone()"));
}

#[test]
fn reads_a_required_consumer_copy_argument_without_cloning() {
    let run = module(&generated(), "run");

    assert!(run.contains(
        "margaret::framework::console::required_argument::required_argument::<crate::budget::Budget,>(matches,\"budget\").and_then(|serve_input_0|"
    ));
    assert!(!run.contains(".clone()"));
}

#[test]
fn reads_an_optional_copy_argument_as_a_copied_option() {
    assert!(module(&generated(), "run").contains(
        "margaret::framework::console::optional_argument::optional_argument::<::std::num::NonZeroU32,>(matches,\"spare-connections\").map(|serve_input_2|"
    ));
}

#[test]
fn moves_a_copy_serve_input_shared_by_two_singletons_without_cloning() {
    let source = module(
        &generated(),
        "container/build/construct_report_budget_report_budget",
    );

    assert!(source.contains(
        "crate::connection_limits::ConnectionLimits::create(serve_input_0,serve_input_1)"
    ));
    assert!(source.contains("serve_input_0,serve_input_1,serve_input_2,"));
    assert!(!source.contains(".clone()"));
}
