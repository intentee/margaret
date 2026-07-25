use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_console_argument_codegen::argument_registration::argument_registration;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::console_command::ConsoleCommand;

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

fn command_arm(command: &ConsoleCommand, container: &Ident) -> TokenStream {
    let name = &command.name;
    let accessor = &command.accessor;
    let matches_binding = if command.arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };
    let values = command.arguments.iter().map(argument_value);
    let accessor_access = quote! { #container.#accessor(#(#values),*).await };

    if command.takes_token {
        quote! {
            Some((#name, #matches_binding)) => {
                let cancellation_token = match margaret::framework::service::install::install() {
                    Ok(cancellation_token) => cancellation_token,
                    Err(error) => {
                        return margaret::framework::console::report_failure::report_failure(error);
                    }
                };

                #accessor_access.run(cancellation_token).await
            }
        }
    } else {
        quote! {
            Some((#name, #matches_binding)) => #accessor_access.run().await,
        }
    }
}

pub(crate) fn render(
    commands: &[ConsoleCommand],
    serves: bool,
    has_models: bool,
    servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
) -> TokenStream {
    let dispatches_asynchronously = !commands.is_empty() || serves;
    let container = if dispatches_asynchronously {
        format_ident!("container")
    } else {
        format_ident!("_container")
    };
    let run_asyncness = if dispatches_asynchronously {
        quote! { async }
    } else {
        quote! {}
    };
    let subcommands = commands.iter().map(subcommand_registration);
    let arms = commands
        .iter()
        .map(|command| command_arm(command, &container));

    let schema_registration = if has_models {
        quote! {
            .subcommand(clap::Command::new("schema"))
        }
    } else {
        quote! {}
    };

    let schema_arm = if has_models {
        quote! {
            Some(("schema", _matches)) => {
                println!(
                    "{}",
                    margaret::framework::model::render_postgres::render_postgres(&super::schema::schema())
                );

                margaret::framework::console::command_outcome::CommandOutcome::Succeeded
            }
        }
    } else {
        quote! {}
    };

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

            quote! {
                .arg(clap::Arg::new(#address_argument).long(#address_argument).required(true))
                .arg(clap::Arg::new(#url_argument).long(#url_argument).required(true))
                .arg(clap::Arg::new(#uploads_argument).long(#uploads_argument).action(clap::ArgAction::SetTrue))
                .arg(clap::Arg::new(#upload_dir_argument).long(#upload_dir_argument).required(false).requires(#uploads_argument))
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
                margaret::framework::service::dispatch_serve::dispatch_serve(
                    margaret::framework::service::install::install,
                    |cancellation_token| super::serve::serve(#container, matches, cancellation_token),
                )
                .await
            }
        }
    } else {
        quote! {}
    };

    quote! {
        pub #run_asyncness fn run<Arguments, Argument>(
            #container: &super::container::Container,
            args: Arguments,
        ) -> margaret::framework::console::command_outcome::CommandOutcome
        where
            Arguments: IntoIterator<Item = Argument>,
            Argument: Into<std::ffi::OsString> + Clone,
        {
            let mut command = clap::Command::new(env!("CARGO_PKG_NAME"))
                #(#subcommands)*
                #serve_registration
                #schema_registration;

            match command.try_get_matches_from_mut(args) {
                Ok(matches) => match matches.subcommand() {
                    #(#arms)*
                    #serve_arm
                    #schema_arm
                    _ => margaret::framework::console::print_help::print_help(&mut command),
                },
                Err(error) => {
                    margaret::framework::console::outcome_for_clap_error::outcome_for_clap_error(error)
                }
            }
        }
    }
}
