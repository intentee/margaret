use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::reported_construction::reported_construction;
use margaret_container::serve_input_binding::ServeInputBinding;
use margaret_container::slotted_serve_input::SlottedServeInput;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_origin_source::ServerOriginSource;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::server_uploads::ServerUploads;
use margaret_http_codegen::serves_spiffe::serves_spiffe;
use margaret_serve_input_codegen::chained_read::ChainedRead;
use margaret_serve_input_codegen::chained_reads::chained_reads;
use margaret_serve_input_codegen::serve_input_read::serve_input_read;
use margaret_serve_input_codegen::serve_input_registration::serve_input_registration;
use margaret_serve_input_codegen::spiffe_client_requirements::SpiffeClientRequirements;
use margaret_service_codegen::serve_invocation_read::ServeInvocationRead;

use crate::console_command::ConsoleCommand;
use crate::framework_command::FrameworkCommand;
use crate::invocation_dispatch::InvocationDispatch;
use crate::serve_dispatch::ServeDispatch;

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
    let svid_active = spiffe_secured
        || SpiffeClientRequirements::of(serve_inputs.iter().map(|slotted| &slotted.input)).any();
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

    let name = FrameworkCommand::Serve.name();

    quote! {
        .subcommand(clap::Command::new(#name)#(#http_server_arguments)*#spiffe_arguments #(#service_arguments)*)
    }
}

fn command_execution(command: &ConsoleCommand, bindings: &ContainerBindings) -> TokenStream {
    let naming = bindings.serve_input_naming();
    let arguments: Vec<ServeInputBinding> = command
        .serve_inputs
        .iter()
        .map(|SlottedServeInput { slot, .. }| {
            let binding = naming.ident(*slot);

            ServeInputBinding {
                slot: *slot,
                value: quote! { #binding },
            }
        })
        .collect();
    let field_name = command.accessor.to_string();
    let constructed = reported_construction(
        &bindings.construction_invocation(
            &command.construction_root,
            &field_name,
            &bindings.construction_arguments(&field_name, &arguments),
        ),
        bindings.construction_outcome(&command.construction_root),
    );

    if command.takes_token {
        let run_call = if command.is_async {
            quote! { constructed.run(cancellation_token).await }
        } else {
            quote! { constructed.run(cancellation_token) }
        };

        quote! {
            margaret::framework::service::dispatch_serve::dispatch_serve(
                margaret::framework::service::install::install,
                |cancellation_token| async move {
                    let constructed = #constructed;

                    margaret::framework::console::command_outcome::CommandOutcome::from_user_result(
                        #run_call,
                    )
                },
            )
            .await
        }
    } else {
        let run_call = if command.is_async {
            quote! { constructed.run().await }
        } else {
            quote! { constructed.run() }
        };

        quote! {
            let constructed = #constructed;

            margaret::framework::console::command_outcome::CommandOutcome::from_user_result(
                #run_call,
            )
        }
    }
}

fn command_arm(
    command: &ConsoleCommand,
    bindings: &ContainerBindings,
    dispatch: InvocationDispatch,
) -> TokenStream {
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
    let naming = bindings.serve_input_naming();
    let reads: Vec<ChainedRead> = command
        .serve_inputs
        .iter()
        .map(|SlottedServeInput { input, slot }| {
            let binding = naming.ident(*slot);

            ChainedRead {
                binding: quote! { #binding },
                read: serve_input_read(input),
            }
        })
        .collect();
    let chain = chained_reads(&reads, &dispatch.run(&command_execution(command, bindings)));

    quote! {
        Some((#name, #matches_binding)) => #chain,
    }
}

fn serve_arm(serve: &ServeDispatch, dispatch: InvocationDispatch) -> TokenStream {
    match serve {
        ServeDispatch::Served(ServeInvocationRead { binding, chain }) => {
            let run = dispatch.run(&quote! {
                margaret::framework::service::dispatch_serve::dispatch_serve(
                    margaret::framework::service::install::install,
                    |cancellation_token| super::serve::serve(serve_invocation, cancellation_token),
                )
                .await
            });

            let name = FrameworkCommand::Serve.name();

            quote! {
                Some((#name, #binding)) => #chain.map(|serve_invocation| #run),
            }
        }
        ServeDispatch::Unserved => TokenStream::new(),
    }
}

fn schema_tokens(has_models: bool, dispatch: InvocationDispatch) -> SchemaTokens {
    if !has_models {
        return SchemaTokens {
            arm: TokenStream::new(),
            registration: TokenStream::new(),
        };
    }

    let run = dispatch.run(&quote! {
        println!(
            "{}",
            margaret::framework::model::render_postgres::render_postgres(&super::schema::SCHEMA)
        );

        margaret::framework::console::command_outcome::CommandOutcome::Succeeded
    });

    let name = FrameworkCommand::Schema.name();

    SchemaTokens {
        arm: quote! {
            Some((#name, _matches)) => ::std::result::Result::Ok(#run),
        },
        registration: quote! {
            .subcommand(clap::Command::new(#name))
        },
    }
}

struct SchemaTokens {
    arm: TokenStream,
    registration: TokenStream,
}

pub(crate) fn render(
    commands: &[ConsoleCommand],
    serve: &ServeDispatch,
    has_models: bool,
    http_servers: &[HttpServer],
    serve_inputs: &[SlottedServeInput],
    bindings: &ContainerBindings,
) -> Vec<GeneratedModuleTokens> {
    let serves = matches!(serve, ServeDispatch::Served(_));
    let dispatch = if serves
        || commands.iter().any(|command| {
            command.takes_token
                || command.is_async
                || bindings.construction_is_async(&command.construction_root)
        }) {
        InvocationDispatch::Asynchronous
    } else {
        InvocationDispatch::Synchronous
    };
    let run_asyncness = dispatch.run_asyncness();
    let subcommands = commands.iter().map(subcommand_registration);
    let arms = commands
        .iter()
        .map(|command| command_arm(command, bindings, dispatch))
        .collect::<Vec<TokenStream>>();
    let SchemaTokens {
        arm: schema_arm,
        registration: schema_registration,
    } = schema_tokens(has_models, dispatch);
    let serve_registration = if serves {
        serve_registration(http_servers, serve_inputs)
    } else {
        quote! {}
    };
    let serve_arm = serve_arm(serve, dispatch);
    let invoked = dispatch.invoked();
    let invocation_type = dispatch.invocation_type();
    let too_many_lines = too_many_lines_allow();

    vec![
        GeneratedModuleTokens::new(
            ModulePath::root("run").child("invocation"),
            quote! {
                pub enum Invocation {
                    Help,
                    Run(#invocation_type),
                }
            },
        ),
        GeneratedModuleTokens::new(
            ModulePath::root("run"),
            quote! {
                mod invocation;

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

                    match margaret::framework::console::parsed_arguments::parsed_arguments(&mut command, args)
                        .and_then(|matches| match matches.subcommand() {
                            #(#arms)*
                            #serve_arm
                            #schema_arm
                            _ => ::std::result::Result::Ok(self::invocation::Invocation::Help),
                        })
                    {
                        Ok(self::invocation::Invocation::Run(command_run)) => #invoked,
                        Ok(self::invocation::Invocation::Help) => {
                            margaret::framework::console::print_help::print_help(&mut command)
                        }
                        Err(outcome) => outcome,
                    }
                }
            },
        ),
    ]
}
