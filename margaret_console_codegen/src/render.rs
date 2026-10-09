use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_container::slotted_serve_input::SlottedServeInput;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_origin_source::ServerOriginSource;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::server_uploads::ServerUploads;
use margaret_http_codegen::serves_spiffe::serves_spiffe;
use margaret_serve_input_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_serve_input_codegen::serve_input_read::serve_input_read;
use margaret_serve_input_codegen::serve_input_registration::serve_input_registration;

use crate::console_command::ConsoleCommand;

fn transport_argument_registration(server: &HttpServer) -> TokenStream {
    let transport_argument = server.transport_argument();
    let value_parser = match server.transport_policy() {
        ServerTransportPolicy::Negotiable => quote! {
            clap::value_parser!(margaret::framework::service::transport_choice::TransportChoice)
        },
        ServerTransportPolicy::PinnedSpiffeMtls => quote! {
            margaret::framework::service::transport_choice::TransportChoice::pinned_to_spiffe_mtls()
        },
    };

    quote! {
        .arg(
            clap::Arg::new(#transport_argument)
                .long(#transport_argument)
                .required(true)
                .value_parser(#value_parser)
        )
    }
}

fn subcommand_registration(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let about = match &command.description {
        Some(description) => quote! { .about(#description) },
        None => quote! {},
    };
    let arguments = command
        .serve_inputs
        .iter()
        .map(|slotted| serve_input_registration(&slotted.input));

    quote! {
        .subcommand(clap::Command::new(#name)#about #(#arguments)*)
    }
}

fn serve_registration(
    http_servers: &[HttpServer],
    serve_inputs: &[SlottedServeInput],
) -> TokenStream {
    let spiffe_secured = serves_spiffe(http_servers);
    let svid_active =
        spiffe_secured || has_spiffe_http_client(serve_inputs.iter().map(|slotted| &slotted.input));
    let service_arguments = serve_inputs
        .iter()
        .map(|slotted| serve_input_registration(&slotted.input));
    let http_server_arguments = http_servers.iter().map(|server| {
        let address_argument = server.address_argument();
        let url_argument = match server.origin() {
            ServerOriginSource::Argument => {
                let url_argument = server.url_argument();

                quote! {
                    .arg(clap::Arg::new(#url_argument).long(#url_argument).required(true).value_parser(clap::value_parser!(margaret::framework::server_origin::server_origin::ServerOrigin)))
                }
            }
            ServerOriginSource::Issuer { .. } => quote! {},
        };
        let upload_dir_argument = match server.uploads() {
            ServerUploads::Accepted => {
                let upload_dir_argument = server.upload_dir_argument();

                quote! {
                    .arg(clap::Arg::new(#upload_dir_argument).long(#upload_dir_argument).required(true).value_parser(clap::value_parser!(::std::path::PathBuf)))
                }
            }
            ServerUploads::Refused => quote! {},
        };
        let transport_argument =
            spiffe_secured.then(|| transport_argument_registration(server));

        quote! {
            .arg(clap::Arg::new(#address_argument).long(#address_argument).required(true))
            #url_argument
            #upload_dir_argument
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

fn command_arm(command: &ConsoleCommand, bindings: &ContainerBindings) -> TokenStream {
    let name = &command.name;
    let matches_binding = if command
        .serve_inputs
        .iter()
        .any(|slotted| slotted.input.reads_clap_matches())
    {
        quote! { matches }
    } else {
        quote! { _matches }
    };
    let values: Vec<ServeInputBinding> = command
        .serve_inputs
        .iter()
        .map(|SlottedServeInput { input, slot }| ServeInputBinding {
            slot: *slot,
            value: serve_input_read(input),
        })
        .collect();
    let construction = bindings.construction_invocation(
        &command.construction_root,
        &command.accessor.to_string(),
        &values,
    );
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
                    margaret::framework::model::render_postgres::render_postgres(&super::schema::SCHEMA)
                );

                margaret::framework::console::command_outcome::CommandOutcome::Succeeded
            }
        }
    } else {
        quote! {}
    };

    SchemaTokens { arm, registration }
}

struct SchemaTokens {
    arm: TokenStream,
    registration: TokenStream,
}

pub(crate) fn render(
    commands: &[ConsoleCommand],
    serves: bool,
    has_models: bool,
    http_servers: &[HttpServer],
    serve_inputs: &[SlottedServeInput],
    bindings: &ContainerBindings,
) -> TokenStream {
    let dispatches_asynchronously = serves
        || commands.iter().any(|command| {
            command.takes_token
                || command.is_async
                || bindings.construction_is_async(&command.construction_root)
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
    } = schema_tokens(has_models);

    let serve_registration = if serves {
        serve_registration(http_servers, serve_inputs)
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

    let too_many_lines = too_many_lines_allow();
    quote! {
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
    }
}
