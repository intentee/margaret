use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_console_argument_codegen::console_argument_read::console_argument_read;
use margaret_environment_variable_codegen::environment_variable_read::environment_variable_read;

use crate::serve_input::ServeInput;

#[must_use]
pub fn serve_input_read(input: &ServeInput) -> TokenStream {
    match input {
        ServeInput::ConsoleArgument(argument) => console_argument_read(argument),
        ServeInput::EnvironmentVariable(variable) => environment_variable_read(variable),
        ServeInput::SpiffeHttpClient => {
            let spiffe_http_client = spiffe_http_client_ident();

            quote! { #spiffe_http_client.clone() }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
    use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::serve_input::ServeInput;

    use super::serve_input_read;

    fn value() -> InputValue {
        InputValue {
            required: true,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
            weaving: WeavingKind::BorrowedStr,
        }
    }

    fn collapsed(input: &ServeInput) -> String {
        serve_input_read(input)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_console_argument_reads_from_the_clap_matches() {
        assert!(
            collapsed(&ServeInput::ConsoleArgument(ConsoleArgument::Flag {
                name: "loud".to_string(),
            }))
            .contains(r#"matches.get_flag("loud")"#)
        );
    }

    #[test]
    fn an_environment_variable_reads_from_the_process_environment() {
        assert!(
            collapsed(&ServeInput::EnvironmentVariable(EnvironmentVariable {
                name: EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable"),
                value: value(),
            }))
            .contains("margaret::framework::environment_variable::read_required::read_required")
        );
    }

    #[test]
    fn a_spiffe_http_client_clones_the_serve_local() {
        assert_eq!(
            collapsed(&ServeInput::SpiffeHttpClient),
            "spiffe_http_client.clone()"
        );
    }
}
