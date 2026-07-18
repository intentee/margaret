use proc_macro2::TokenStream;
use quote::quote;

use margaret_console_argument_codegen::argument_registration::argument_registration;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_cookie_policy::ServerCookiePolicy;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::console_command::ConsoleCommand;

fn cookie_argument_registration(server: &HttpServer) -> TokenStream {
    let cookie_domain_argument = server.cookie_domain_argument();
    let cookie_insecure_argument = server.cookie_insecure_argument();

    quote! {
        .arg(clap::Arg::new(#cookie_domain_argument).long(#cookie_domain_argument).required(true))
        .arg(clap::Arg::new(#cookie_insecure_argument).long(#cookie_insecure_argument).action(clap::ArgAction::SetTrue))
    }
}

fn transport_argument_registration(server: &HttpServer) -> TokenStream {
    let transport_argument = server.transport_argument();
    let allowed_values = match server.transport_policy() {
        ServerTransportPolicy::Negotiable => quote! { ["plain", "spiffe_mtls"] },
        ServerTransportPolicy::PinnedSpiffeMtls => quote! { ["spiffe_mtls"] },
    };

    quote! {
        .arg(
            clap::Arg::new(#transport_argument)
                .long(#transport_argument)
                .required(true)
                .value_parser(#allowed_values)
        )
    }
}

fn subcommand_registration(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let about = match &command.description {
        Some(description) => quote! { .about(#description) },
        None => quote! {},
    };
    let arguments = command.arguments.iter().map(argument_registration);

    quote! {
        .subcommand(clap::Command::new(#name)#about #(#arguments)*)
    }
}

fn command_arm(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let accessor = &command.accessor;
    let matches_binding = if command.arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };
    let values = command.arguments.iter().map(argument_value);
    let accessor_access = quote! { container.#accessor().await };

    if command.takes_token {
        quote! {
            Some((#name, #matches_binding)) => {
                let cancellation_token = match margaret_service::install::install() {
                    Ok(cancellation_token) => cancellation_token,
                    Err(error) => {
                        return margaret_console::report_failure::report_failure(error);
                    }
                };

                #accessor_access.run(#(#values,)* cancellation_token).await
            }
        }
    } else {
        quote! {
            Some((#name, #matches_binding)) => #accessor_access.run(#(#values),*).await,
        }
    }
}

pub(crate) fn render(
    commands: &[ConsoleCommand],
    serves: bool,
    servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
) -> TokenStream {
    let subcommands = commands.iter().map(subcommand_registration);
    let arms = commands.iter().map(command_arm);

    let serve_registration = if serves {
        let spiffe_secured = serves_spiffe(servers);
        let service_arguments = serve_arguments.iter().map(argument_registration);
        let server_arguments = servers.iter().map(|server| {
            let address_argument = server.address_argument();
            let url_argument = server.url_argument();
            let uploads_argument = server.uploads_argument();
            let upload_dir_argument = server.upload_dir_argument();
            let transport_argument =
                spiffe_secured.then(|| transport_argument_registration(server));
            let cookie_arguments = matches!(server.cookie_policy(), ServerCookiePolicy::Cookied)
                .then(|| cookie_argument_registration(server));

            quote! {
                .arg(clap::Arg::new(#address_argument).long(#address_argument).required(true))
                .arg(clap::Arg::new(#url_argument).long(#url_argument).required(true))
                .arg(clap::Arg::new(#uploads_argument).long(#uploads_argument).action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new(#upload_dir_argument).long(#upload_dir_argument).required(false).requires(#uploads_argument))
                #cookie_arguments
                #transport_argument
            }
        });
        let spiffe_arguments = spiffe_secured.then(|| {
            quote! {
                .arg(clap::Arg::new("spiffe-trust-domain").long("spiffe-trust-domain").required(true))
                .arg(clap::Arg::new("spire-agent-addr").long("spire-agent-addr").required(true))
            }
        });

        quote! {
            .subcommand(clap::Command::new("serve")#(#server_arguments)*#spiffe_arguments #(#service_arguments)*)
        }
    } else {
        quote! {}
    };

    let serve_arm = if serves {
        quote! {
            Some(("serve", matches)) => {
                margaret_service::dispatch_serve::dispatch_serve(
                    margaret_service::install::install,
                    |cancellation_token| super::serve::serve(container, matches, cancellation_token),
                )
                .await
            }
        }
    } else {
        quote! {}
    };

    quote! {
        pub async fn run<Arguments, Argument>(
            container: &super::container::Container,
            args: Arguments,
        ) -> margaret_console::command_outcome::CommandOutcome
        where
            Arguments: IntoIterator<Item = Argument>,
            Argument: Into<std::ffi::OsString> + Clone,
        {
            let mut command = clap::Command::new(env!("CARGO_PKG_NAME"))
                #(#subcommands)*
                #serve_registration;

            match command.try_get_matches_from_mut(args) {
                Ok(matches) => match matches.subcommand() {
                    #(#arms)*
                    #serve_arm
                    _ => margaret_console::print_help::print_help(&mut command),
                },
                Err(error) => {
                    margaret_console::outcome_for_clap_error::outcome_for_clap_error(error)
                }
            }
        }
    }
}
