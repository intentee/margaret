use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn without_formatting(source: &str) -> String {
    source
        .split_whitespace()
        .collect::<String>()
        .replace(",>", ">")
}

fn fixture() -> ModuleSources {
    let generated =
        generate_fixture("type_aware_environment_variables").expect("the fixture generates");
    ModuleSources {
        construction: without_formatting(
            generated_module_source(&generated, "container/build/serve_arguments")
                .expect("the bootstrap arguments module is generated"),
        ),
        run: without_formatting(
            generated_module_source(&generated, "run").expect("the run module is generated"),
        ),
        serve: without_formatting(
            generated_module_source(&generated, "serve").expect("the serve module is generated"),
        ),
    }
}

struct ModuleSources {
    construction: String,
    run: String,
    serve: String,
}

#[test]
fn declares_each_variable_with_its_canonical_value_type() {
    let construction = fixture().construction;

    assert!(construction.contains(":std::string::String,"));
    assert!(construction.contains(":std::path::PathBuf,"));
    assert!(construction.contains(":u16,"));
    assert!(construction.contains(":bool,"));
    assert!(construction.contains(":::std::option::Option<u16>,"));
    assert!(construction.contains(":::std::option::Option<std::string::String>,"));
}

#[test]
fn reads_a_required_variable_through_the_required_reader() {
    assert!(fixture().serve.contains(
        "margaret::framework::environment_variable::read_required::read_required::<std::string::String>(\"DATABASE_URL\")"
    ));
}

#[test]
fn reads_an_optional_variable_through_the_optional_reader() {
    assert!(fixture().serve.contains(
        "margaret::framework::environment_variable::read_optional::read_optional::<u16>(\"OPTIONAL_RETRIES\")"
    ));
}

#[test]
fn reads_a_boolean_variable_as_an_ordinary_parsed_value() {
    assert!(fixture().serve.contains(
        "margaret::framework::environment_variable::read_required::read_required::<bool>(\"WORKER_DEBUG\")"
    ));
}

#[test]
fn reports_a_failed_read_and_leaves_the_command() {
    assert!(fixture().serve.contains(
        "Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error);}"
    ));
}

#[test]
fn registers_no_clap_argument_for_a_variable() {
    let run = fixture().run;

    assert!(!run.contains("DATABASE_URL"));
    assert!(!run.contains("WORKER_DEBUG"));
}
