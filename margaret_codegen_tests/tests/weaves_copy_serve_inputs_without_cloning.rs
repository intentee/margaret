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
    assert!(module(&generated(), "run").contains(
        "matches.get_one::<std::num::NonZeroU32>(\"max-connections\"){Some(value)=>*value"
    ));
}

#[test]
fn reads_a_required_consumer_copy_argument_without_cloning() {
    assert!(
        module(&generated(), "run")
            .contains("matches.get_one::<crate::budget::Budget>(\"budget\"){Some(value)=>*value")
    );
}

#[test]
fn reads_an_optional_copy_argument_as_a_copied_option() {
    assert!(
        module(&generated(), "run")
            .contains("matches.get_one::<std::num::NonZeroU32>(\"spare-connections\").copied()")
    );
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
