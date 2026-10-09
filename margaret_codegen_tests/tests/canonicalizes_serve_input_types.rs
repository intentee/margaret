use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_source::generated_source;

#[test]
fn canonicalizes_imported_serve_input_types_across_generated_modules() {
    let generated =
        generate_fixture("type_aware_console_arguments").expect("the fixture generates");
    let source: String = generated_source(&generated).split_whitespace().collect();

    assert!(source.contains("clap::value_parser!(::std::string::String)"));
    assert!(source.contains("clap::value_parser!(::std::path::PathBuf)"));
    assert!(source.contains(r#"matches.get_one::<::std::path::PathBuf>("root")"#));
    assert!(source.contains("::std::option::Option<::std::string::String>"));
}
