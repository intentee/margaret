use proc_macro2::TokenStream;
use quote::quote;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
use margaret_input_weaving::weaving_kind::WeavingKind;

use crate::route_url_input::RouteUrlInput;
use crate::serve_input_key::ServeInputKey;

#[derive(Clone, Debug, PartialEq)]
pub enum ServeInput {
    ConsoleArgument(ConsoleArgument),
    EnvironmentVariable(EnvironmentVariable),
    RouteUrl(RouteUrlInput),
    SpiffeHttpClient,
}

impl ServeInput {
    #[must_use]
    pub fn field_type(&self) -> TokenStream {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.field_type(),
            ServeInput::EnvironmentVariable(variable) => variable.value.field_type(),
            ServeInput::RouteUrl(_) => quote! { ::std::string::String },
            ServeInput::SpiffeHttpClient => quote! { reqwest::Client },
        }
    }

    #[must_use]
    pub fn is_shareable(&self) -> bool {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.is_shareable(),
            ServeInput::EnvironmentVariable(_)
            | ServeInput::RouteUrl(_)
            | ServeInput::SpiffeHttpClient => true,
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.name(),
            ServeInput::EnvironmentVariable(variable) => variable.name.as_str(),
            ServeInput::RouteUrl(route) => &route.path,
            ServeInput::SpiffeHttpClient => "spiffe_http_client",
        }
    }

    #[must_use]
    pub fn reads_server_origins(&self) -> bool {
        matches!(self, ServeInput::RouteUrl(_))
    }

    #[must_use]
    pub fn reads_clap_matches(&self) -> bool {
        match self {
            ServeInput::ConsoleArgument(_) => true,
            ServeInput::EnvironmentVariable(_)
            | ServeInput::RouteUrl(_)
            | ServeInput::SpiffeHttpClient => false,
        }
    }

    #[must_use]
    pub fn slot_key(&self) -> ServeInputKey {
        match self {
            ServeInput::ConsoleArgument(argument) => ServeInputKey::ConsoleArgument {
                name: argument.name().to_string(),
            },
            ServeInput::EnvironmentVariable(variable) => ServeInputKey::EnvironmentVariable {
                name: variable.name.as_str().to_string(),
            },
            ServeInput::RouteUrl(route) => ServeInputKey::RouteUrl(route.clone()),
            ServeInput::SpiffeHttpClient => ServeInputKey::SpiffeHttpClient,
        }
    }

    #[must_use]
    pub fn weaving(&self) -> WeavingKind {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.weaving(),
            ServeInput::EnvironmentVariable(variable) => variable.value.weaving.clone(),
            ServeInput::RouteUrl(_) | ServeInput::SpiffeHttpClient => WeavingKind::Cloned,
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
    use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;

    use crate::route_url_input::RouteUrlInput;
    use crate::serve_input_key::ServeInputKey;

    use super::ServeInput;

    fn collapsed(tokens: &TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    fn string_value(required: bool, weaving: WeavingKind) -> InputValue {
        InputValue {
            required,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
            weaving,
        }
    }

    fn console_argument() -> ServeInput {
        ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: "label".to_string(),
            value: string_value(true, WeavingKind::BorrowedStr),
        })
    }

    fn environment_variable() -> ServeInput {
        ServeInput::EnvironmentVariable(EnvironmentVariable {
            name: EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable"),
            value: string_value(false, WeavingKind::Cloned),
        })
    }

    #[test]
    fn a_console_argument_delegates_its_shape_to_the_argument() {
        let input = console_argument();

        assert_eq!(collapsed(&input.field_type()), "std::string::String");
        assert_eq!(input.name(), "label");
        assert_eq!(input.weaving(), WeavingKind::BorrowedStr);
        assert!(input.is_shareable());
        assert!(input.reads_clap_matches());
        assert_eq!(
            input.slot_key(),
            ServeInputKey::ConsoleArgument {
                name: "label".to_string()
            }
        );
    }

    #[test]
    fn a_positional_console_argument_never_shares_a_slot() {
        let input = ServeInput::ConsoleArgument(ConsoleArgument::Positional {
            id: "name".to_string(),
            value: string_value(true, WeavingKind::BorrowedStr),
        });

        assert!(!input.is_shareable());
    }

    #[test]
    fn an_environment_variable_delegates_its_shape_to_its_value() {
        let input = environment_variable();

        assert_eq!(
            collapsed(&input.field_type()),
            "::std::option::Option<std::string::String>"
        );
        assert_eq!(input.name(), "DATABASE_URL");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert_eq!(
            input.slot_key(),
            ServeInputKey::EnvironmentVariable {
                name: "DATABASE_URL".to_string()
            }
        );
    }

    #[test]
    fn a_spiffe_http_client_is_a_cloned_reqwest_client() {
        let input = ServeInput::SpiffeHttpClient;

        assert_eq!(collapsed(&input.field_type()), "reqwest::Client");
        assert_eq!(input.name(), "spiffe_http_client");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(!input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::SpiffeHttpClient);
    }

    fn callback() -> RouteUrlInput {
        RouteUrlInput {
            path: "/sign-in/callback".to_string(),
            server: "public".to_string(),
        }
    }

    #[test]
    fn a_route_url_is_a_cloned_string_composed_from_the_server_origins() {
        let input = ServeInput::RouteUrl(callback());

        assert_eq!(collapsed(&input.field_type()), "::std::string::String");
        assert_eq!(input.name(), "/sign-in/callback");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::RouteUrl(callback()));
    }

    #[test]
    fn a_console_argument_and_an_environment_variable_of_the_same_name_key_apart() {
        let argument = ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: "DATABASE_URL".to_string(),
            value: string_value(true, WeavingKind::BorrowedStr),
        });

        assert_ne!(argument.slot_key(), environment_variable().slot_key());
    }
}
