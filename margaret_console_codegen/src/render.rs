use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_serve_input_codegen::serve_input::ServeInput;
use margaret_serve_input_codegen::serve_input_read::serve_input_read;
use margaret_serve_input_codegen::serve_input_registration::serve_input_registration;

use crate::console_command::ConsoleCommand;
use crate::console_plan::ConsolePlan;
use crate::serve_command::ServeCommand;

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
    let arguments = command.serve_inputs.iter().map(serve_input_registration);

    quote! {
        .subcommand(clap::Command::new(#name)#about #(#arguments)*)
    }
}

fn serve_registration(command: &ServeCommand) -> TokenStream {
    let service_arguments = command.serve_inputs.iter().map(serve_input_registration);
    let http_server_arguments = command.http_servers.iter().map(|server| {
        let address_argument = server.address_argument();
        let body_limit_argument = server.body_limit_argument();
        let url_argument = server.url_argument();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let transport_argument = command
            .registers_transport_arguments
            .then(|| transport_argument_registration(server));

        quote! {
            .arg(clap::Arg::new(#address_argument).long(#address_argument).required(true))
            .arg(
                clap::Arg::new(#body_limit_argument)
                    .long(#body_limit_argument)
                    .required(false)
                    .value_parser(clap::value_parser!(usize))
            )
            .arg(clap::Arg::new(#url_argument).long(#url_argument).required(true))
            .arg(clap::Arg::new(#uploads_argument).long(#uploads_argument).action(clap::ArgAction::SetTrue))
            .arg(clap::Arg::new(#upload_dir_argument).long(#upload_dir_argument).required(false).requires(#uploads_argument))
            #transport_argument
        }
    });
    let spiffe_arguments = command.registers_spiffe_arguments.then(|| {
        quote! {
            .arg(clap::Arg::new("spiffe-trust-domain").long("spiffe-trust-domain").required(true))
            .arg(clap::Arg::new("spire-agent-addr").long("spire-agent-addr").required(true))
        }
    });

    quote! {
        .subcommand(clap::Command::new("serve")#(#http_server_arguments)*#spiffe_arguments #(#service_arguments)*)
    }
}

fn command_arm(command: &ConsoleCommand, bindings: &ContainerBindings) -> TokenStream {
    let name = &command.name;
    let matches_binding = if command
        .serve_inputs
        .iter()
        .any(ServeInput::reads_clap_matches)
    {
        quote! { matches }
    } else {
        quote! { _matches }
    };
    let values: Vec<ServeInputBinding> = command
        .serve_inputs
        .iter()
        .zip(&command.serve_input_slots)
        .map(|(argument, slot)| ServeInputBinding {
            slot: *slot,
            value: serve_input_read(argument),
        })
        .collect();
    let construction = bindings.construction_invocation(&command.accessor.to_string(), &values);
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

        quote! {
            Some((#name, #matches_binding)) => {
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
        }
    } else {
        let run_call = if command.is_async {
            quote! { #accessor_access.run().await }
        } else {
            quote! { #accessor_access.run() }
        };

        quote! {
            Some((#name, #matches_binding)) => {
                margaret::framework::console::command_outcome::CommandOutcome::from_user_result(
                    #run_call,
                )
            }
        }
    }
}

struct SchemaTokens {
    arm: TokenStream,
    registration: TokenStream,
}

pub(crate) struct RenderedConsole {
    pub(crate) run: TokenStream,
}

fn schema_tokens(has_models: bool) -> SchemaTokens {
    let registration = if has_models {
        quote! {
            .subcommand(clap::Command::new("schema"))
        }
    } else {
        quote! {}
    };

    let arm = if has_models {
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

    SchemaTokens { arm, registration }
}

pub(crate) fn render(plan: &ConsolePlan, bindings: &ContainerBindings) -> RenderedConsole {
    let commands = &plan.commands;
    let dispatches_asynchronously = plan.serve.is_some()
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
    let arms = commands
        .iter()
        .map(|command| command_arm(command, bindings))
        .collect::<Vec<TokenStream>>();

    let SchemaTokens {
        arm: schema_arm,
        registration: schema_registration,
    } = schema_tokens(plan.has_models);

    let serve_registration = match &plan.serve {
        Some(command) => serve_registration(command),
        None => quote! {},
    };

    let serve_arm = if plan.serve.is_some() {
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

    let too_many_lines = too_many_lines_allow();
    let run = quote! {
        #too_many_lines
        pub #run_asyncness fn run<Arguments, Argument>(
            args: Arguments,
        ) -> margaret::framework::console::command_outcome::CommandOutcome
        where
            Arguments: IntoIterator<Item = Argument>,
            Argument: Into<std::ffi::OsString> + Clone,
        {
            let mut command = clap::Command::new(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
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
                    margaret::framework::console::outcome_for_clap_error::outcome_for_clap_error(&error)
                }
            }
        }
    };

    RenderedConsole { run }
}
