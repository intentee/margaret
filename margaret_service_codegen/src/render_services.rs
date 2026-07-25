use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_console_argument_codegen::required_flag_read::required_flag_read;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::framework_service::FrameworkService;
use crate::serve_console_arguments::ServeConsoleArguments;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;
use crate::spiffe_activation::SpiffeActivation;
use crate::startup_singleton::StartupSingleton;

fn transport_expression(server: &HttpServer, spiffe_secured: bool) -> TokenStream {
    if !spiffe_secured {
        return quote! { margaret::framework::http::transport_config::TransportConfig::Plain };
    }

    match server.transport_policy() {
        ServerTransportPolicy::PinnedSpiffeMtls => quote! {
            margaret::framework::http::transport_config::TransportConfig::MutualTls {
                server_config: spiffe_server_config.clone(),
            }
        },
        ServerTransportPolicy::Negotiable => {
            let transport_argument = server.transport_argument();

            quote! {
                match matches.get_one::<String>(#transport_argument).map(String::as_str) {
                    Some("spiffe_mtls") => margaret::framework::http::transport_config::TransportConfig::MutualTls {
                        server_config: spiffe_server_config.clone(),
                    },
                    _ => margaret::framework::http::transport_config::TransportConfig::Plain,
                }
            }
        }
    }
}

fn svid_identity_prelude(
    SpiffeActivation {
        client_active,
        server_active,
    }: SpiffeActivation,
) -> TokenStream {
    if !server_active && !client_active {
        return quote! {};
    }

    let spiffe_trust_domain =
        required_flag_read(&quote! { String }, "spiffe-trust-domain", &quote! { value.clone() });
    let spire_agent_addr =
        required_flag_read(&quote! { String }, "spire-agent-addr", &quote! { value.clone() });

    let bundle_constructor = if server_active && client_active {
        quote! { margaret::framework::spiffe_svid_bundle::SvidBundle }
    } else if server_active {
        quote! { margaret::framework::spiffe_svid_server::SvidServerBundle }
    } else {
        quote! { margaret::framework::spiffe_svid_client::SvidClientBundle }
    };

    let server_config = server_active.then(|| {
        quote! {
            let spiffe_server_config = ::std::sync::Arc::new(spiffe_bundle.server_config());
        }
    });

    let client_readiness = client_active.then(|| {
        quote! {
            let mut spiffe_client_readiness = spiffe_bundle.client_readiness();
        }
    });

    let http_client = client_active.then(|| {
        let spiffe_http_client = spiffe_http_client_ident();

        quote! {
            let #spiffe_http_client = match spiffe_bundle.reqwest_client() {
                Ok(client) => client,
                Err(error) => return margaret::framework::console::report_failure::report_failure(error),
            };
        }
    });

    quote! {
        margaret::framework::spiffe_svid::install_default_crypto_provider::install_default_crypto_provider();

        let spiffe_bundle = #bundle_constructor::new(
            margaret::framework::spiffe_svid::SvidServiceBundleParams {
                spiffe_trust_domain: #spiffe_trust_domain,
                spire_agent_addr: #spire_agent_addr,
            },
        );

        #server_config
        #client_readiness
        #http_client
    }
}

fn spiffe_identity_gate(SpiffeActivation { client_active, .. }: SpiffeActivation) -> TokenStream {
    if !client_active {
        return quote! {};
    }

    quote! {
        let mut spiffe_identity_manager = trzcina::ServiceManager::default();

        if let Err(error) = spiffe_identity_manager.register_bundle(spiffe_bundle).await {
            return margaret::framework::console::report_failure::report_failure(error);
        }

        let spiffe_identity_running =
            spiffe_identity_manager.start(cancellation_token.clone());

        match spiffe_client_readiness
            .wait_until_ready(&cancellation_token)
            .await
        {
            margaret::framework::sync_holder::sync_holder_presence::SyncHolderPresence::Present => {}
            margaret::framework::sync_holder::sync_holder_presence::SyncHolderPresence::Cancelled => {
                return margaret::framework::service::run_all::run_all(
                    ::std::vec![spiffe_identity_running],
                    trzcina::ServiceShutdownOptions::default(),
                )
                .await;
            }
        }
    }
}

fn server_manager_setup(
    servers: &[HttpServer],
    has_views: bool,
    registers_services: bool,
    activation: SpiffeActivation,
    bindings: &ContainerBindings,
    server_console_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    views_console_arguments: &[ConsoleArgument],
) -> TokenStream {
    let svid_bundle_in_main = activation.server_active && !activation.client_active;
    let manager_binding = if registers_services {
        quote! { mut manager }
    } else {
        quote! { manager }
    };

    let bundle_services_registration = svid_bundle_in_main.then(|| {
        quote! {
            match margaret::framework::service::bundle_services::bundle_services(spiffe_bundle).await {
                Ok(services) => bundle_services.extend(services),
                Err(outcome) => return outcome,
            }
        }
    });

    if servers.is_empty() {
        return quote! {
            let #manager_binding = trzcina::ServiceManager::default();
        };
    }

    let origins = servers.iter().map(|server| {
        let origin_variable = format_ident!("origin_{}", server.name());
        let origin_read = required_flag_read(
            &quote! { String },
            &server.url_argument(),
            &quote! { value.clone().into() },
        );

        quote! {
            let #origin_variable: ::std::sync::Arc<str> = #origin_read;
        }
    });

    let origin_arguments = servers.iter().map(|server| {
        let origin_variable = format_ident!("origin_{}", server.name());

        quote! { #origin_variable.clone() }
    });

    let empty_arguments: Vec<ConsoleArgument> = Vec::new();
    let views_argument = has_views.then(|| quote! { , &views });
    let assemblies = servers.iter().map(|server| {
        let function_name = server.function_name();
        let name = server.name();
        let address_argument = server.address_argument();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let transport = transport_expression(server, activation.server_active);
        let server_borrows = bindings.console_borrows(
            server_console_arguments
                .get(server.name())
                .unwrap_or(&empty_arguments),
        );

        quote! {
            margaret::framework::service::server_assembly::ServerAssembly {
                address_argument: #address_argument,
                name: #name,
                routes: super::http::#function_name::#function_name(container, #(#server_borrows)* &routes #views_argument).await,
                transport: #transport,
                upload_dir_argument: #upload_dir_argument,
                uploads_argument: #uploads_argument,
            }
        }
    });
    let assemblies = vec_literal_tokens(assemblies);

    let bundle_services_binding = if svid_bundle_in_main {
        quote! { mut bundle_services }
    } else {
        quote! { bundle_services }
    };
    let views_setup = has_views.then(|| {
        let views_borrows = bindings.console_borrows(views_console_arguments);

        quote! {
            let views = ::std::sync::Arc::new(super::views::build::build(container, #(#views_borrows)*).await);
        }
    });

    quote! {
        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );
        #views_setup
        let servers = #assemblies;

        let #bundle_services_binding: ::std::vec::Vec<
            ::std::boxed::Box<dyn trzcina::Service>,
        > = ::std::vec::Vec::new();

        #bundle_services_registration

        let #manager_binding = match margaret::framework::service::serve_application::serve_application(
            matches,
            servers,
            margaret::framework::service::resolved_services::ResolvedServices {
                services: bundle_services,
            },
        )
        .await
        {
            Ok(manager) => manager,
            Err(outcome) => return outcome,
        };
    }
}

fn missed_tick_behavior_method(behavior: &Option<Path>) -> TokenStream {
    match behavior {
        Some(behavior) => quote! {
            fn missed_tick_behavior(&self) -> tokio::time::MissedTickBehavior {
                #behavior
            }
        },
        None => quote! {},
    }
}

fn adapter(unit: &ServiceUnit) -> TokenStream {
    match &unit.kind {
        ServiceKind::Service => service_adapter(unit),
        ServiceKind::Ticker { behavior, interval } => ticker_adapter(unit, behavior, interval),
    }
}

fn ticker_adapter(unit: &ServiceUnit, behavior: &Option<Path>, interval: &Path) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let runner = format_ident!("{}", unit.runner);
    let missed_tick_behavior_method = missed_tick_behavior_method(behavior);
    let (token_binding, call) = if unit.takes_token {
        (
            quote! { cancellation_token },
            quote! { self.inner.#runner(cancellation_token).await },
        )
    } else {
        (
            quote! { _cancellation_token },
            quote! { self.inner.#runner().await },
        )
    };

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
        }

        #[async_trait::async_trait]
        impl trzcina::Ticker for #name {
            fn tick_interval(&self) -> std::time::Duration {
                #interval
            }

            #missed_tick_behavior_method

            async fn handle_tick(
                &mut self,
                #token_binding: tokio_util::sync::CancellationToken,
                _tick_context: trzcina::TickContext,
            ) -> anyhow::Result<()> {
                #call.map_err(anyhow::Error::from)
            }
        }
    }
}

fn service_adapter(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let runner = format_ident!("{}", unit.runner);
    let (token_binding, call) = if unit.takes_token {
        (
            quote! { cancellation_token },
            quote! { self.inner.#runner(cancellation_token).await? },
        )
    } else {
        (
            quote! { _cancellation_token },
            quote! { self.inner.#runner().await? },
        )
    };

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
        }

        #[async_trait::async_trait]
        impl trzcina::Service for #name {
            async fn run(
                self: Box<Self>,
                #token_binding: tokio_util::sync::CancellationToken,
            ) -> anyhow::Result<()> {
                #call;

                Ok(())
            }
        }
    }
}

fn woven_arguments(unit: &ServiceUnit, bindings: &ContainerBindings) -> Vec<TokenStream> {
    bindings.console_weaves_owned(bindings.console_arguments(&unit.concrete_path))
}

fn registration(unit: &ServiceUnit, bindings: &ContainerBindings) -> TokenStream {
    let name = adapter_ident(unit);
    let accessor = format_ident!("{}", unit.field_name);
    let woven = woven_arguments(unit, bindings);

    quote! {
        manager.register_service(#name { inner: container.#accessor(#(#woven),*).await });
    }
}

fn adapter_ident(unit: &ServiceUnit) -> Ident {
    format_ident!("{}", unit.type_name)
}

fn startup_constructions(
    startup_singletons: &[StartupSingleton],
    bindings: &ContainerBindings,
) -> TokenStream {
    let constructions = startup_singletons.iter().map(|singleton| {
        let accessor = format_ident!("{}", singleton.field_name);
        let woven =
            bindings.console_weaves_owned(bindings.console_arguments(&singleton.concrete_path));

        quote! {
            if let Err(error) = container.#accessor(#(#woven),*).await.ensure_reachable().await {
                return margaret::framework::console::report_failure::report_failure(error);
            }
        }
    });

    quote! { #(#constructions)* }
}

fn serve_prelude(serve_arguments: &[ConsoleArgument], bindings: &ContainerBindings) -> TokenStream {
    let resolutions = serve_arguments.iter().map(|argument| {
        let ident = console_argument_ident(bindings.console_slot(&argument.slot_key()));
        let value = argument_value(argument);

        quote! { let #ident = #value; }
    });

    quote! { #(#resolutions)* }
}

pub fn render_services(
    index: &AttributeIndex,
    servers: &[HttpServer],
    has_views: bool,
    bindings: &ContainerBindings,
    ServeConsoleArguments {
        serve_arguments,
        server_console_arguments,
        views_console_arguments,
    }: ServeConsoleArguments,
    framework_services: &[FrameworkService],
    startup_singletons: &[StartupSingleton],
) -> Result<GeneratedModuleTokens, ServiceCodegenError> {
    let mut units = service_units(index)?;

    units.extend(framework_services.iter().map(ServiceUnit::from_framework));

    let activation = SpiffeActivation {
        client_active: has_spiffe_http_client(serve_arguments),
        server_active: serves_spiffe(servers),
    };

    let adapters = units.iter().map(adapter);
    let startup = startup_constructions(startup_singletons, bindings);
    let registrations = units.iter().map(|unit| registration(unit, bindings));
    let identity_prelude = svid_identity_prelude(activation);
    let identity_gate = spiffe_identity_gate(activation);
    let manager_setup = server_manager_setup(
        servers,
        has_views,
        !units.is_empty(),
        activation,
        bindings,
        server_console_arguments,
        views_console_arguments,
    );
    let prelude = serve_prelude(serve_arguments, bindings);
    let matches_binding = if servers.is_empty() && serve_arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };

    let run_manager = if activation.client_active {
        quote! {
            let spiffe_application_running = manager.start(cancellation_token.clone());

            margaret::framework::service::run_all::run_all(
                ::std::vec![spiffe_identity_running, spiffe_application_running],
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    } else {
        quote! {
            margaret::framework::service::run::run(
                manager,
                cancellation_token,
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    let tokens = quote! {
        #(#adapters)*

        pub async fn serve(
            container: &super::container::Container,
            #matches_binding: &clap::ArgMatches,
            cancellation_token: tokio_util::sync::CancellationToken,
        ) -> margaret::framework::console::command_outcome::CommandOutcome {
            #identity_prelude
            #identity_gate
            #prelude
            #startup
            #manager_setup

            #(#registrations)*

            #run_manager
        }
    };

    Ok(GeneratedModuleTokens::new("serve", tokens))
}
