use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

#[test]
fn weaves_each_serve_input_category_with_its_type_appropriate_operation() {
    let generated =
        generate_fixture("type_aware_console_arguments").expect("the fixture generates");
    let construction: String =
        generated_module_source(&generated, "container/build/serve_arguments")
            .expect("the bootstrap arguments module is generated")
            .split_whitespace()
            .collect();
    let run: String = generated_module_source(&generated, "run")
        .expect("the run module is generated")
        .split_whitespace()
        .collect();

    assert!(construction.contains(":std::string::String,"));
    assert!(construction.contains(":std::path::PathBuf,"));
    assert!(construction.contains(":u16,"));
    assert!(construction.contains(":bool,"));
    assert!(construction.contains(":::std::option::Option<u16>,"));
    assert!(construction.contains(":::std::option::Option<::std::string::String>,"));

    assert!(run.contains(
        r#"margaret::framework::console::required_argument::required_argument::<::std::string::String,>(matches,"label")"#
    ));
    assert!(run.contains(
        r#"margaret::framework::console::required_argument::required_argument::<::std::path::PathBuf,>(matches,"root")"#
    ));
    assert!(run.contains(
        r#"margaret::framework::console::required_argument::required_argument::<u16,>(matches,"retries")"#
    ));
    assert!(run.contains(
        r#"margaret::framework::console::flag_argument::flag_argument(matches,"verbose",)"#
    ));
    assert!(run.contains(
        r#"margaret::framework::console::optional_argument::optional_argument::<u16,>(matches,"optional-retries")"#
    ));
    assert!(run.contains(
        r#"margaret::framework::console::optional_argument::optional_argument::<::std::string::String,>(matches,"note")"#
    ));
}
