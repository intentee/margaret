use proc_macro2::TokenStream;
use quote::quote;

use margaret_console_argument_codegen::console_argument_registration::console_argument_registration;

use crate::serve_input::ServeInput;

#[must_use]
pub fn serve_input_registration(input: &ServeInput) -> TokenStream {
    match input {
        ServeInput::ConsoleArgument(argument) => console_argument_registration(argument),
        ServeInput::EnvironmentVariable(_)
        | ServeInput::SpiffeHttpClient
        | ServeInput::SpiffeWebSocketClient => quote! {},
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

    use super::serve_input_registration;

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
        serve_input_registration(input)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_console_argument_registers_a_clap_argument() {
        assert!(
            collapsed(&ServeInput::ConsoleArgument(ConsoleArgument::Named {
                name: "label".to_string(),
                value: value(),
            }))
            .contains(r#"clap::Arg::new("label")"#)
        );
    }

    #[test]
    fn an_environment_variable_registers_nothing() {
        assert!(
            collapsed(&ServeInput::EnvironmentVariable(EnvironmentVariable {
                name: EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable"),
                value: value(),
            }))
            .is_empty()
        );
    }

    #[test]
    fn a_spiffe_http_client_registers_nothing() {
        assert!(collapsed(&ServeInput::SpiffeHttpClient).is_empty());
    }

    #[test]
    fn a_spiffe_websocket_client_registers_nothing() {
        assert!(collapsed(&ServeInput::SpiffeWebSocketClient).is_empty());
    }
}
