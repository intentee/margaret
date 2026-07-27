use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::has_spiffe_http_client::has_spiffe_http_client;
use margaret_console_argument_codegen::owned_weave::owned_weave;
use margaret_console_argument_codegen::required_flag_read::required_flag_read;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::framework_service::FrameworkService;
use crate::serve_console_arguments::ServeConsoleArguments;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_unit_origin::ServiceUnitOrigin;
use crate::service_units::service_units;
use crate::spiffe_activation::SpiffeActivation;

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
                    Some("plain") => margaret::framework::http::transport_config::TransportConfig::Plain,
                    Some("spiffe_mtls") => margaret::framework::http::transport_config::TransportConfig::MutualTls {
                        server_config: spiffe_server_config.clone(),
                    },
                    Some(_) | None => {
                        return margaret::framework::console::command_outcome::CommandOutcome::Failed;
                    }
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

    let spiffe_trust_domain = required_flag_read(
        &quote! { String },
        "spiffe-trust-domain",
        &quote! { value.clone() },
    );
    let spire_agent_addr = required_flag_read(
        &quote! { String },
        "spire-agent-addr",
        &quote! { value.clone() },
    );

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
            let spiffe_client_readiness = spiffe_bundle.client_readiness();
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

fn gated_registration(inner: &TokenStream, client_active: bool) -> TokenStream {
    if client_active {
        quote! {
            manager.register_service(
                margaret::framework::spiffe_svid_client::readiness_gated_service::ReadinessGatedService::new(
                    spiffe_client_readiness.clone(),
                    #inner,
                ),
            );
        }
    } else {
        quote! {
            manager.register_service(#inner);
        }
    }
}

fn bundle_registration(
    SpiffeActivation {
        client_active,
        server_active,
    }: SpiffeActivation,
) -> TokenStream {
    if !server_active && !client_active {
        return quote! {};
    }

    quote! {
        if let Err(error) = manager.register_bundle(spiffe_bundle).await {
            return margaret::framework::console::report_failure::report_failure(error);
        }
    }
}

fn server_registration(
    servers: &[HttpServer],
    has_views: bool,
    activation: SpiffeActivation,
) -> TokenStream {
    if servers.is_empty() {
        return quote! {};
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

    let views_argument = has_views.then(|| quote! { , &views });
    let assemblies = servers.iter().map(|server| {
        let function_name = server.function_name();
        let name = server.name();
        let address_argument = server.address_argument();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let transport = transport_expression(server, activation.server_active);
        let routes = if server.routes_are_async() {
            quote! {
                super::http::#function_name::#function_name(container, &routes #views_argument).await
            }
        } else {
            quote! {
                super::http::#function_name::#function_name(container, &routes #views_argument)
            }
        };

        quote! {
            margaret::framework::service::server_assembly::ServerAssembly {
                address_argument: #address_argument,
                name: #name,
                routes: #routes,
                transport: #transport,
                upload_dir_argument: #upload_dir_argument,
                uploads_argument: #uploads_argument,
            }
        }
    });
    let assemblies = vec_literal_tokens(assemblies);

    let views_setup = has_views.then(|| {
        let built = quote! { super::views::build::build(container).await };

        quote! {
            let views = ::std::sync::Arc::new(#built);
        }
    });

    let register_server = gated_registration(&quote! { server_service }, activation.client_active);

    quote! {
        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );
        #views_setup
        let servers = #assemblies;

        let server_services = match margaret::framework::service::serve_application::serve_application(
            matches,
            servers,
        ) {
            Ok(server_services) => server_services,
            Err(outcome) => return outcome,
        };

        for server_service in server_services {
            #register_server
        }
    }
}

fn missed_tick_behavior_method(behavior: &Option<CanonicalPath>) -> TokenStream {
    match behavior {
        Some(behavior) => {
            let behavior = path_tokens(behavior);

            quote! {
                fn missed_tick_behavior(&self) -> tokio::time::MissedTickBehavior {
                    #behavior
                }
            }
        }
        None => quote! {},
    }
}

fn adapter(unit: &ServiceUnit) -> TokenStream {
    match &unit.kind {
        ServiceKind::Service => service_adapter(unit),
        ServiceKind::Ticker { behavior, interval } => ticker_adapter(unit, behavior, interval),
    }
}

fn runner_outcome(unit: &ServiceUnit, call: &TokenStream) -> TokenStream {
    match unit.origin {
        ServiceUnitOrigin::Framework => quote! {
            #call?;

            ::std::result::Result::Ok(())
        },
        ServiceUnitOrigin::User => quote! {
            let outcome: margaret::framework::anyhow::Result<()> = #call;

            outcome
        },
    }
}

fn ticker_adapter(
    unit: &ServiceUnit,
    behavior: &Option<CanonicalPath>,
    interval: &CanonicalPath,
) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let interval = path_tokens(interval);
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
    let outcome = runner_outcome(unit, &call);

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
            ) -> margaret::framework::anyhow::Result<()> {
                #outcome
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
            quote! { self.inner.#runner(cancellation_token).await },
        )
    } else {
        (
            quote! { _cancellation_token },
            quote! { self.inner.#runner().await },
        )
    };
    let outcome = runner_outcome(unit, &call);

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
        }

        #[async_trait::async_trait]
        impl trzcina::Service for #name {
            async fn run(
                self: Box<Self>,
                #token_binding: tokio_util::sync::CancellationToken,
            ) -> margaret::framework::anyhow::Result<()> {
                #outcome
            }
        }
    }
}

fn registration(
    unit: &ServiceUnit,
    bindings: &ContainerBindings,
    client_active: bool,
) -> TokenStream {
    let name = adapter_ident(unit);
    let container = format_ident!("container");
    let access = bindings.accessor_invocation(&container, &unit.field_name);
    let inner = quote! { #name { inner: #access } };

    gated_registration(&inner, client_active)
}

fn adapter_ident(unit: &ServiceUnit) -> Ident {
    format_ident!("{}", unit.type_name)
}

fn serve_inputs(
    serve_arguments: &[ConsoleArgument],
    bindings: &ContainerBindings,
) -> Result<(TokenStream, Vec<TokenStream>), ServiceCodegenError> {
    let mut resolutions = Vec::with_capacity(serve_arguments.len());
    let mut construction_arguments = Vec::with_capacity(serve_arguments.len());

    for argument in serve_arguments {
        let slot = bindings.console_slot(&argument.slot_key())?;
        let ident = console_argument_ident(slot);
        let value = argument_value(argument);

        resolutions.push(quote! { let #ident = #value; });
        construction_arguments.push(owned_weave(argument, slot, true));
    }

    Ok((quote! { #(#resolutions)* }, construction_arguments))
}

pub fn render_services(
    index: &AttributeIndex,
    servers: &[HttpServer],
    has_views: bool,
    bindings: &ContainerBindings,
    ServeConsoleArguments {
        serve_arguments,
        server_console_arguments: _,
        views_console_arguments: _,
    }: ServeConsoleArguments,
    framework_services: &[FrameworkService],
) -> Result<GeneratedModuleTokens, ServiceCodegenError> {
    let mut units = service_units(index)?;

    units.extend(framework_services.iter().map(ServiceUnit::from_framework));

    let activation = SpiffeActivation {
        client_active: has_spiffe_http_client(serve_arguments),
        server_active: serves_spiffe(servers),
    };

    let adapters = units.iter().map(adapter);
    let unit_registrations = units
        .iter()
        .map(|unit| registration(unit, bindings, activation.client_active));
    let identity_prelude = svid_identity_prelude(activation);
    let bundle_registration = bundle_registration(activation);
    let server_registration = server_registration(servers, has_views, activation);
    let (prelude, construction_arguments) = serve_inputs(serve_arguments, bindings)?;
    let construction_invocation = bindings.serve_invocation(&construction_arguments);
    let construction = quote! {
        let container = match #construction_invocation {
            Ok(container) => container,
            Err(error) => {
                return margaret::framework::console::report_failure::report_failure(error);
            }
        };
        let container = &container;
    };
    let matches_binding = if servers.is_empty() && serve_arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };

    let tokens = quote! {
        #(#adapters)*

        pub async fn serve(
            #matches_binding: &clap::ArgMatches,
            cancellation_token: tokio_util::sync::CancellationToken,
        ) -> margaret::framework::console::command_outcome::CommandOutcome {
            #identity_prelude
            #prelude
            #construction

            let mut manager = trzcina::ServiceManager::default();

            #bundle_registration
            #server_registration
            #(#unit_registrations)*

            margaret::framework::service::run::run(
                manager,
                cancellation_token,
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    Ok(GeneratedModuleTokens::new("serve", tokens))
}
