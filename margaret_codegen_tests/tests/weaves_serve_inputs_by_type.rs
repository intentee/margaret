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
    let serve: String = generated_module_source(&generated, "serve")
        .expect("the serve module is generated")
        .split_whitespace()
        .collect();

    assert!(construction.contains(":std::string::String,"));
    assert!(construction.contains(":std::path::PathBuf,"));
    assert!(construction.contains(":u16,"));
    assert!(construction.contains(":bool,"));
    assert!(construction.contains(":::std::option::Option<u16>,"));
    assert!(construction.contains(":::std::option::Option<::std::string::String>,"));

    assert!(serve.contains(
        r#"matches.get_one::<::std::string::String>("label"){Some(value)=>value.clone()"#
    ));
    assert!(
        serve.contains(
            r#"matches.get_one::<::std::path::PathBuf>("root"){Some(value)=>value.clone()"#
        )
    );
    assert!(serve.contains("Some(value)=>*value"));
    assert!(serve.contains("matches.get_flag"));
}
