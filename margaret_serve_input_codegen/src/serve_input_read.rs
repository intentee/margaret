use quote::quote;

use margaret_codegen_tokens::routes_ident::routes_ident;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_codegen_tokens::spiffe_issuer_request_client_ident::spiffe_issuer_request_client_ident;
use margaret_console_argument_codegen::console_argument_read::console_argument_read;
use margaret_environment_variable_codegen::environment_variable_read::environment_variable_read;

use crate::read_expression::ReadExpression;
use crate::route_url_read::route_url_read;
use crate::serve_input::ServeInput;

#[must_use]
pub fn serve_input_read(input: &ServeInput) -> ReadExpression {
    match input {
        ServeInput::ConsoleArgument(argument) => {
            ReadExpression::Fallible(console_argument_read(argument))
        }
        ServeInput::EnvironmentVariable(variable) => {
            ReadExpression::Fallible(environment_variable_read(variable))
        }
        ServeInput::Prepared(preparation) => {
            let call = preparation.call();

            ReadExpression::Fallible(quote! {
                #call.map_err(margaret::framework::console::report_failure::report_failure)
            })
        }
        ServeInput::RouteUrl(route) => ReadExpression::Fallible(route_url_read(route)),
        ServeInput::Routes => {
            let routes = routes_ident();

            ReadExpression::Infallible(quote! { ::std::sync::Arc::clone(&#routes) })
        }
        ServeInput::SpiffeHttpClient => {
            let spiffe_http_client = spiffe_http_client_ident();

            ReadExpression::Infallible(quote! { #spiffe_http_client.clone() })
        }
        ServeInput::SpiffeIssuerRequestClient => {
            let spiffe_issuer_request_client = spiffe_issuer_request_client_ident();

            ReadExpression::Infallible(
                quote! { ::std::sync::Arc::clone(&#spiffe_issuer_request_client) },
            )
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

    use crate::preparation::Preparation;
    use crate::read_expression::ReadExpression;
    use crate::route_url_input::RouteUrlInput;
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
        match serve_input_read(input) {
            ReadExpression::Fallible(expression) => format!("fallible:{expression}"),
            ReadExpression::Infallible(expression) => format!("infallible:{expression}"),
        }
        .split_whitespace()
        .collect()
    }

    #[test]
    fn a_console_argument_reads_from_the_clap_matches() {
        assert!(
            collapsed(&ServeInput::ConsoleArgument(ConsoleArgument::Flag {
                name: "loud".to_string(),
            }))
            .starts_with(r#"fallible:margaret::framework::console::flag_argument::flag_argument(matches,"loud")"#)
        );
    }

    #[test]
    fn an_environment_variable_reads_from_the_process_environment() {
        assert!(
            collapsed(&ServeInput::EnvironmentVariable(EnvironmentVariable {
                name: EnvironmentVariableName::new("DATABASE_URL").expect("the name is usable"),
                value: value(),
            }))
            .starts_with(
                "fallible:margaret::framework::environment_variable::read_required::read_required"
            )
        );
    }

    #[test]
    fn a_route_url_composes_the_url_from_the_server_origin() {
        assert!(
            collapsed(&ServeInput::RouteUrl(RouteUrlInput {
                path: "/callback".to_string(),
                server: "public".to_string(),
            }))
            .starts_with(
                "fallible:margaret::framework::server_origin::route_url::RouteUrl::parse(margaret::framework::http::literal_url::literal_url(&origin_public"
            )
        );
    }

    #[test]
    fn a_prepared_value_reports_its_failure_through_the_parse_chain() {
        assert_eq!(
            collapsed(&ServeInput::Prepared(Preparation {
                arguments: vec![CanonicalPath::new(vec![
                    "crate".to_string(),
                    "ENDPOINTS".to_string(),
                ])],
                function: CanonicalPath::new(vec![
                    "crate".to_string(),
                    "Metadata".to_string(),
                    "of_provider".to_string(),
                ]),
                prepared: CanonicalPath::new(vec!["crate".to_string(), "Metadata".to_string()]),
            })),
            "fallible:crate::Metadata::of_provider(crate::ENDPOINTS).map_err(margaret::framework::console::report_failure::report_failure)"
        );
    }

    #[test]
    fn the_routes_share_the_serve_local_through_an_arc_so_they_are_built_once() {
        assert_eq!(
            collapsed(&ServeInput::Routes),
            "infallible:::std::sync::Arc::clone(&routes)"
        );
    }

    #[test]
    fn a_spiffe_http_client_clones_the_serve_local() {
        assert_eq!(
            collapsed(&ServeInput::SpiffeHttpClient),
            "infallible:spiffe_http_client.clone()"
        );
    }

    #[test]
    fn a_spiffe_issuer_request_client_shares_the_serve_local_built_once() {
        assert_eq!(
            collapsed(&ServeInput::SpiffeIssuerRequestClient),
            "infallible:::std::sync::Arc::clone(&spiffe_issuer_request_client)"
        );
    }
}
