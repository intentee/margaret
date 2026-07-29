use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::name_allocator::NameAllocator;
use margaret_console_argument_codegen::argument_registration::argument_registration;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_container::console_argument_binding::ConsoleArgumentBinding;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::command_builder::COMMAND_BUILDER;
use crate::console_command::ConsoleCommand;
use crate::run_entry_point::RUN_ENTRY_POINT;

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

fn serve_registration(
    http_servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
) -> TokenStream {
    let spiffe_secured = serves_spiffe(http_servers);
    let svid_active = spiffe_secured || has_spiffe_http_client(serve_arguments);
    let service_arguments = serve_arguments.iter().map(argument_registration);
    let http_server_arguments = http_servers.iter().map(|server| {
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
    let spiffe_arguments = svid_active.then(|| {
        quote! {
            .arg(clap::Arg::new("spiffe-trust-domain").long("spiffe-trust-domain").required(true))
            .arg(clap::Arg::new("spire-agent-addr").long("spire-agent-addr").required(true))
        }
    });

    quote! {
        .subcommand(clap::Command::new("serve")#(#http_server_arguments)*#spiffe_arguments #(#service_arguments)*)
    }
}

pub(crate) struct RenderedCommand {
    pub(crate) arm: TokenStream,
    pub(crate) dispatch: TokenStream,
}

fn command_arm(
    command: &ConsoleCommand,
    module: &Ident,
    bindings: &ContainerBindings,
) -> RenderedCommand {
    let name = &command.name;
    let dispatch_parameter = if command.arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };
    let matches_binding = quote! { matches };
    let values: Vec<ConsoleArgumentBinding> = command
        .arguments
        .iter()
        .zip(&command.console_slots)
        .map(|(argument, slot)| ConsoleArgumentBinding {
            slot: *slot,
            value: argument_value(argument),
        })
        .collect();
    let construction = bindings.construction_invocation(&command.accessor.to_string(), &values);
    let construction_is_async = bindings.construction_is_async(&command.accessor.to_string());
    let accessor_access = quote! {
        (match #construction {
            Ok(value) => value,
            Err(error) => {
                return margaret::framework::console::report_failure::report_failure(error);
            }
        })
    };

    if command.takes_token {
        let run_call = if command.is_async {
            quote! { #accessor_access.run(cancellation_token).await }
        } else {
            quote! { #accessor_access.run(cancellation_token) }
        };

        RenderedCommand {
            arm: quote! {
                Some((#name, #matches_binding)) => #module(#matches_binding).await,
            },
            dispatch: quote! {
                async fn #module(
                    #dispatch_parameter: &clap::ArgMatches,
                ) -> margaret::framework::console::command_outcome::CommandOutcome {
                    margaret::framework::service::dispatch_serve::dispatch_serve(
                        margaret::framework::service::install::install,
                        |cancellation_token| async move {
                            margaret::framework::console::command_outcome::CommandOutcome::from_user_result(
                                #run_call,
                            )
                        },
                    )
                    .await
                }
            },
        }
    } else {
        let run_call = if command.is_async {
            quote! { #accessor_access.run().await }
        } else {
            quote! { #accessor_access.run() }
        };

        let dispatch_asyncness = if command.is_async || construction_is_async {
            quote! { async }
        } else {
            quote! {}
        };
        let dispatch_call = if command.is_async || construction_is_async {
            quote! { #module(#matches_binding).await }
        } else {
            quote! { #module(#matches_binding) }
        };

        RenderedCommand {
            arm: quote! {
                Some((#name, #matches_binding)) => #dispatch_call,
            },
            dispatch: quote! {
                #dispatch_asyncness fn #module(
                    #dispatch_parameter: &clap::ArgMatches,
                ) -> margaret::framework::console::command_outcome::CommandOutcome {
                    margaret::framework::console::command_outcome::CommandOutcome::from_user_result(
                        #run_call,
                    )
                }
            },
        }
    }
}

pub(crate) struct RenderedConsole {
    pub(crate) command: TokenStream,
    pub(crate) dispatches: Vec<RenderedCommand>,
    pub(crate) run: TokenStream,
}

pub(crate) fn render(
    commands: &[ConsoleCommand],
    serves: bool,
    has_models: bool,
    http_servers: &[HttpServer],
    serve_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> RenderedConsole {
    let dispatches_asynchronously = serves
        || commands.iter().any(|command| {
            command.takes_token
                || command.is_async
                || bindings.construction_is_async(&command.accessor.to_string())
        });
    let run_asyncness = if dispatches_asynchronously {
        quote! { async }
    } else {
        quote! {}
    };
    let subcommands = commands.iter().map(subcommand_registration);
    let mut dispatch_names = NameAllocator::new();

    dispatch_names.reserve(RUN_ENTRY_POINT);
    dispatch_names.reserve(COMMAND_BUILDER);

    let rendered_commands: Vec<RenderedCommand> = commands
        .iter()
        .map(|command| {
            let module = format_ident!(
                "{}",
                dispatch_names
                    .allocate(&command.accessor.to_string())
                    .field()
            );

            command_arm(command, &module, bindings)
        })
        .collect();
    let arms = rendered_commands.iter().map(|rendered| &rendered.arm);

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
        serve_registration(http_servers, serve_arguments)
    } else {
        quote! {}
    };

    let serve_arm = if serves {
        quote! {
            Some(("serve", matches)) => {
                margaret::framework::service::dispatch_serve::dispatch_serve(
                    margaret::framework::service::install::install,
                            |cancellation_token| super::serve::serve(matches, cancellation_token),
                )
                .await
            }
        }
    } else {
        quote! {}
    };

    let command = quote! {
        #[must_use]
        pub(crate) fn command() -> clap::Command {
            clap::Command::new(env!("CARGO_PKG_NAME"))
                #(#subcommands)*
                #serve_registration
                #schema_registration
        }
    };
    let run = quote! {
        pub #run_asyncness fn run<Arguments, Argument>(
            args: Arguments,
        ) -> margaret::framework::console::command_outcome::CommandOutcome
        where
            Arguments: IntoIterator<Item = Argument>,
            Argument: Into<std::ffi::OsString> + Clone,
        {
            let mut command = super::run::command::command();

            match command.try_get_matches_from_mut(args) {
                Ok(matches) => match matches.subcommand() {
                    #(#arms)*
                    #serve_arm
                    #schema_arm
                    _ => margaret::framework::console::print_help::print_help(&mut command),
                },
                Err(error) => {
                    margaret::framework::console::outcome_for_clap_error::outcome_for_clap_error(&error)
                }
            }
        }
    };

    RenderedConsole {
        command,
        dispatches: rendered_commands,
        run,
    }
}
