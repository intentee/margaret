use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;

#[test]
fn weaves_each_console_argument_category_with_its_type_appropriate_operation() {
    let generated =
        generate_fixture("type_aware_console_arguments").expect("the fixture generates");
    let source: String = generated_module_source(&generated, "http/server_public")
        .expect("the HTTP server module is generated")
        .split_whitespace()
        .collect();

    assert!(source.contains(":&str,"));
    assert!(source.contains(":&::std::path::Path,"));
    assert!(source.contains(":&u16,"));
    assert!(source.contains(":&bool,"));
    assert!(source.contains(":&::std::option::Option<u16>,"));
    assert!(source.contains(":&::std::option::Option<std::string::String>,"));
    assert!(!source.contains(":&std::string::String,"));
    assert!(!source.contains(":&std::path::PathBuf,"));

    assert_eq!(source.matches(".to_owned()").count(), 2);
    assert_eq!(source.matches(".clone()").count(), 1);
    assert_eq!(source.matches("*console_argument_").count(), 3);
}
