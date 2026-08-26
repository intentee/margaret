use margaret_codegen::codegen_error::CodegenError;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_input_weaving::input_weaving_error::InputWeavingError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

#[test]
fn rejects_a_generic_serve_input_value_type() {
    assert!(matches!(
        generate_fixture("generic_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ServeInput {
            source: ServeInputCodegenError::ConsoleArgument {
                source: ConsoleArgumentCodegenError::ValueType {
                    source: InputWeavingError::GenericValueType { site, value_type },
                },
            },
        } if site.owner.to_string() == "crate::Config"
            && site.parameter == "tags"
            && value_type == "Vec < String >"
    ));
}

#[test]
fn rejects_a_non_path_serve_input_value_type() {
    assert!(matches!(
        generate_fixture("non_path_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ServeInput {
            source: ServeInputCodegenError::ConsoleArgument {
                source: ConsoleArgumentCodegenError::ValueType {
                    source: InputWeavingError::UnresolvableValueType { site, value_type },
                },
            },
        } if site.owner.to_string() == "crate::Config"
            && site.parameter == "pair"
            && value_type == "(u8 , u8)"
    ));
}

#[test]
fn rejects_an_unresolvable_serve_input_value_type() {
    assert!(matches!(
        generate_fixture("unresolvable_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ServeInput {
            source: ServeInputCodegenError::ConsoleArgument {
                source: ConsoleArgumentCodegenError::ValueType {
                    source: InputWeavingError::UnresolvableValueType { site, value_type },
                },
            },
        } if site.owner.to_string() == "crate::Config"
            && site.parameter == "widget"
            && value_type == "Widget"
    ));
}
