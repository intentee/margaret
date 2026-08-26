use margaret_codegen::codegen_error::CodegenError;
use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;

#[test]
fn rejects_a_generic_console_argument_value_type() {
    assert!(matches!(
        generate_fixture("generic_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ConsoleArgument {
            source: ConsoleArgumentCodegenError::GenericValueType {
                owner,
                parameter,
                value_type,
            },
        } if owner == "crate::Config"
            && parameter == "tags"
            && value_type == "Vec < String >"
    ));
}

#[test]
fn rejects_a_non_path_console_argument_value_type() {
    assert!(matches!(
        generate_fixture("non_path_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ConsoleArgument {
            source: ConsoleArgumentCodegenError::UnresolvableValueType {
                owner,
                parameter,
                value_type,
            },
        } if owner == "crate::Config"
            && parameter == "pair"
            && value_type == "(u8 , u8)"
    ));
}

#[test]
fn rejects_an_unresolvable_console_argument_value_type() {
    assert!(matches!(
        generate_fixture("unresolvable_console_argument").expect_err("the fixture is rejected"),
        CodegenError::ConsoleArgument {
            source: ConsoleArgumentCodegenError::UnresolvableValueType {
                owner,
                parameter,
                value_type,
            },
        } if owner == "crate::Config"
            && parameter == "widget"
            && value_type == "Widget"
    ));
}
