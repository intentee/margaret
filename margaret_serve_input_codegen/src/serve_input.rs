use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_environment_variable_codegen::environment_variable::EnvironmentVariable;
use margaret_input_weaving::weaving_kind::WeavingKind;
use margaret_umbrella_path::routes_path::routes_path;

use crate::preparation::Preparation;
use crate::route_url_input::RouteUrlInput;
use crate::serve_input_key::ServeInputKey;

#[derive(Clone, Debug, PartialEq)]
pub enum ServeInput {
    ConsoleArgument(ConsoleArgument),
    EnvironmentVariable(EnvironmentVariable),
    Prepared(Preparation),
    RouteUrl(RouteUrlInput),
    Routes,
    SpiffeHttpClient,
    SpiffeIssuerRequestClient,
}

impl ServeInput {
    #[must_use]
    pub fn field_type(&self) -> TokenStream {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.field_type(),
            ServeInput::EnvironmentVariable(variable) => variable.value.field_type(),
            ServeInput::Prepared(Preparation { prepared, .. }) => path_tokens(prepared),
            ServeInput::RouteUrl(_) => {
                quote! { margaret::framework::server_origin::route_url::RouteUrl }
            }
            ServeInput::Routes => {
                let routes = path_tokens(&routes_path());

                quote! { ::std::sync::Arc<#routes> }
            }
            ServeInput::SpiffeHttpClient => quote! { margaret::framework::reqwest::Client },
            ServeInput::SpiffeIssuerRequestClient => quote! {
                ::std::sync::Arc<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient>
            },
        }
    }

    #[must_use]
    pub fn is_shareable(&self) -> bool {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.is_shareable(),
            ServeInput::EnvironmentVariable(_)
            | ServeInput::Prepared(_)
            | ServeInput::RouteUrl(_)
            | ServeInput::Routes
            | ServeInput::SpiffeHttpClient
            | ServeInput::SpiffeIssuerRequestClient => true,
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.name().to_string(),
            ServeInput::EnvironmentVariable(variable) => variable.name.as_str().to_string(),
            ServeInput::Prepared(Preparation { function, .. }) => function.to_string(),
            ServeInput::RouteUrl(route) => route.path.clone(),
            ServeInput::Routes => "routes".to_string(),
            ServeInput::SpiffeHttpClient => "spiffe_http_client".to_string(),
            ServeInput::SpiffeIssuerRequestClient => "spiffe_issuer_request_client".to_string(),
        }
    }

    #[must_use]
    pub fn reads_clap_matches(&self) -> bool {
        match self {
            ServeInput::ConsoleArgument(_) => true,
            ServeInput::EnvironmentVariable(_)
            | ServeInput::Prepared(_)
            | ServeInput::RouteUrl(_)
            | ServeInput::Routes
            | ServeInput::SpiffeHttpClient
            | ServeInput::SpiffeIssuerRequestClient => false,
        }
    }

    #[must_use]
    pub fn reads_server_origins(&self) -> bool {
        matches!(self, ServeInput::RouteUrl(_) | ServeInput::Routes)
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
            ServeInput::Prepared(preparation) => ServeInputKey::Prepared(preparation.clone()),
            ServeInput::RouteUrl(route) => ServeInputKey::RouteUrl(route.clone()),
            ServeInput::Routes => ServeInputKey::Routes,
            ServeInput::SpiffeHttpClient => ServeInputKey::SpiffeHttpClient,
            ServeInput::SpiffeIssuerRequestClient => ServeInputKey::SpiffeIssuerRequestClient,
        }
    }

    #[must_use]
    pub fn weaving(&self) -> WeavingKind {
        match self {
            ServeInput::ConsoleArgument(argument) => argument.weaving(),
            ServeInput::EnvironmentVariable(variable) => variable.value.weaving.clone(),
            ServeInput::Prepared(_)
            | ServeInput::RouteUrl(_)
            | ServeInput::Routes
            | ServeInput::SpiffeHttpClient
            | ServeInput::SpiffeIssuerRequestClient => WeavingKind::Cloned,
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

    use crate::preparation::Preparation;
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

        assert_eq!(collapsed(&input.field_type()), "::std::string::String");
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
            "::std::option::Option<::std::string::String>"
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

        assert_eq!(
            collapsed(&input.field_type()),
            "margaret::framework::reqwest::Client"
        );
        assert_eq!(input.name(), "spiffe_http_client");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(!input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::SpiffeHttpClient);
    }

    #[test]
    fn a_spiffe_issuer_request_client_is_a_shared_handle_built_once_while_serving() {
        let input = ServeInput::SpiffeIssuerRequestClient;

        assert_eq!(
            collapsed(&input.field_type()),
            "::std::sync::Arc<margaret::framework::issuer_request::issuer_request_client::IssuerRequestClient>"
        );
        assert_eq!(input.name(), "spiffe_issuer_request_client");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(!input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::SpiffeIssuerRequestClient);
    }

    #[test]
    fn the_routes_are_a_shared_handle_composed_from_the_server_origins() {
        let input = ServeInput::Routes;

        assert_eq!(
            collapsed(&input.field_type()),
            "::std::sync::Arc<crate::margaret::routes::Routes>"
        );
        assert_eq!(input.name(), "routes");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::Routes);
    }

    fn callback() -> RouteUrlInput {
        RouteUrlInput {
            path: "/sign-in/callback".to_string(),
            server: "public".to_string(),
        }
    }

    #[test]
    fn a_route_url_is_a_cloned_url_parsed_from_the_server_origins() {
        let input = ServeInput::RouteUrl(callback());

        assert_eq!(
            collapsed(&input.field_type()),
            "margaret::framework::server_origin::route_url::RouteUrl"
        );
        assert_eq!(input.name(), "/sign-in/callback");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::RouteUrl(callback()));
    }

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    #[test]
    fn a_prepared_value_is_moved_into_its_only_consumer_under_the_type_it_prepares() {
        let preparation = Preparation {
            arguments: vec![path(&["crate", "ENDPOINTS"])],
            function: path(&["crate", "Metadata", "of_provider"]),
            prepared: path(&["crate", "Metadata"]),
        };
        let input = ServeInput::Prepared(preparation.clone());

        assert_eq!(collapsed(&input.field_type()), "crate::Metadata");
        assert_eq!(input.name(), "crate::Metadata::of_provider");
        assert_eq!(input.weaving(), WeavingKind::Cloned);
        assert!(input.is_shareable());
        assert!(!input.reads_clap_matches());
        assert!(!input.reads_server_origins());
        assert_eq!(input.slot_key(), ServeInputKey::Prepared(preparation));
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
