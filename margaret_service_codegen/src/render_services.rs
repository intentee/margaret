use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::server_origin_ident::server_origin_ident;
use margaret_codegen_tokens::spiffe_http_client_ident::spiffe_http_client_ident;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::required_flag_read::required_flag_read;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_origin_source::ServerOriginSource;
use margaret_http_codegen::server_uploads::ServerUploads;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::first_tick::FirstTick;
use crate::runner_outcome::RunnerOutcome;
use crate::service_kind::ServiceKind;
use crate::service_plan::ServicePlan;
use crate::service_unit::ServiceUnit;
use crate::service_unit_origin::ServiceUnitOrigin;
use crate::spiffe_activation::SpiffeActivation;

fn failed_outcome() -> TokenStream {
    quote! { return margaret::framework::console::command_outcome::CommandOutcome::Failed }
}

fn failed_registration() -> TokenStream {
    quote! {
        return ::std::result::Result::Err(
            margaret::framework::console::command_outcome::CommandOutcome::Failed,
        )
    }
}

fn transport_expression(server: &HttpServer, spiffe_secured: bool) -> TokenStream {
    if !spiffe_secured {
        return quote! { margaret::framework::http::transport_config::TransportConfig::Plain };
    }

    required_flag_read(
        &quote! { margaret::framework::service::transport_choice::TransportChoice },
        &server.transport_argument(),
        &quote! { value.config(spiffe_server_config) },
        &failed_registration(),
    )
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
        &failed_outcome(),
    );
    let spire_agent_addr = required_flag_read(
        &quote! { String },
        "spire-agent-addr",
        &quote! { value.clone() },
        &failed_outcome(),
    );

    let bundle_constructor = if server_active && client_active {
        quote! { margaret::framework::spiffe_svid_bundle::svid_bundle::SvidBundle }
    } else if server_active {
        quote! { margaret::framework::spiffe_svid_server::svid_server_bundle::SvidServerBundle }
    } else {
        quote! { margaret::framework::spiffe_svid_client::svid_client_bundle::SvidClientBundle }
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
        let spiffe_bundle = match #bundle_constructor::new(
            margaret::framework::spiffe_svid::svid_service_bundle_params::SvidServiceBundleParams {
                spiffe_trust_domain: #spiffe_trust_domain,
                spire_agent_addr: #spire_agent_addr,
            },
        ) {
            Ok(bundle) => bundle,
            Err(error) => return margaret::framework::console::report_failure::report_failure(error),
        };

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

fn served_routes(servers: &[HttpServer]) -> TokenStream {
    if servers.is_empty() {
        return TokenStream::new();
    }

    let origins = servers.iter().map(|server| {
        let origin = server_origin_ident(server.name());
        let serialization = match server.origin() {
            ServerOriginSource::Argument => {
                let origin_read = required_flag_read(
                    &quote! { margaret::framework::http::server_origin::ServerOrigin },
                    &server.url_argument(),
                    &quote! { value.origin.ascii_serialization() },
                    &failed_outcome(),
                );

                quote! { #origin_read }
            }
            ServerOriginSource::Issuer { endpoints } => {
                let endpoints = path_tokens(endpoints);

                quote! { #endpoints.issuer_origin }
            }
        };

        quote! {
            let #origin: ::std::sync::Arc<str> = ::std::sync::Arc::from(#serialization);
        }
    });
    let origin_arguments = servers.iter().map(|server| {
        let origin = server_origin_ident(server.name());

        quote! { ::std::sync::Arc::clone(&#origin) }
    });

    quote! {
        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );
    }
}

fn server_registration(
    servers: &[HttpServer],
    has_views: bool,
    activation: SpiffeActivation,
) -> TokenStream {
    let views_argument = has_views.then(|| quote! { , &views });
    let assemblies = servers.iter().map(|server| {
        let function_name = server.function_name();
        let address_argument = server.address_argument();
        let uploads = match server.uploads() {
            ServerUploads::Accepted => {
                let directory_argument = server.upload_dir_argument();

                quote! {
                    margaret::framework::service::server_uploads::ServerUploads::Accepted {
                        directory_argument: #directory_argument,
                    }
                }
            }
            ServerUploads::Refused => {
                quote! { margaret::framework::service::server_uploads::ServerUploads::Refused }
            }
        };
        let transport = transport_expression(server, activation.server_active);
        let routes = quote! {
            super::http::#function_name::#function_name(container, routes #views_argument)
        };

        quote! {
            margaret::framework::service::server_assembly::ServerAssembly {
                address_argument: #address_argument,
                routes: #routes,
                transport: #transport,
                uploads: #uploads,
            }
        }
    });
    let assemblies = vec_literal_tokens(assemblies);

    let views_setup = has_views.then(|| {
        quote! {
            let views = ::std::sync::Arc::new(super::views::build::build(container));
        }
    });

    let register_server = gated_registration(&quote! { server_service }, activation.client_active);

    quote! {
        #views_setup
        let servers = #assemblies;

        let server_services = margaret::framework::service::serve_application::serve_application(
            matches,
            servers,
        )?;

        for server_service in server_services {
            #register_server
        }

        ::std::result::Result::Ok(())
    }
}

fn missed_tick_behavior_method(behavior: Option<&CanonicalPath>) -> TokenStream {
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

fn first_tick_timing_method(first_tick: FirstTick) -> TokenStream {
    match first_tick {
        FirstTick::AfterInterval => quote! {
            fn first_tick_timing(&self) -> trzcina::FirstTickTiming {
                trzcina::FirstTickTiming::AfterInterval
            }
        },
        FirstTick::Immediate => quote! {},
    }
}

fn adapter(unit: &ServiceUnit) -> TokenStream {
    match &unit.kind {
        ServiceKind::Service => service_adapter(unit),
        ServiceKind::Ticker {
            behavior,
            first_tick,
            interval,
        } => ticker_adapter(unit, behavior.as_ref(), *first_tick, interval),
    }
}

fn runner_result(unit: &ServiceUnit, call: &TokenStream) -> TokenStream {
    match unit.origin {
        ServiceUnitOrigin::Framework {
            outcome: RunnerOutcome::Fallible,
        } => quote! {
            #call?;

            ::std::result::Result::Ok(())
        },
        ServiceUnitOrigin::Framework {
            outcome: RunnerOutcome::Infallible,
        } => quote! {
            #call;

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
    behavior: Option<&CanonicalPath>,
    first_tick: FirstTick,
    interval: &CanonicalPath,
) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let interval = path_tokens(interval);
    let missed_tick_behavior_method = missed_tick_behavior_method(behavior);
    let first_tick_timing_method = first_tick_timing_method(first_tick);
    let token_binding = token_binding(unit);
    let outcome = runner_outcome(unit);

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
        }

        #[async_trait::async_trait]
        impl trzcina::Ticker for #name {
            fn tick_interval(&self) -> std::time::Duration {
                #interval
            }

            #first_tick_timing_method

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

fn token_binding(unit: &ServiceUnit) -> TokenStream {
    if unit.takes_token {
        quote! { cancellation_token }
    } else {
        quote! { _cancellation_token }
    }
}

fn runner_outcome(unit: &ServiceUnit) -> TokenStream {
    let runner = format_ident!("{}", unit.runner);
    let arguments = if unit.takes_token {
        quote! { cancellation_token }
    } else {
        quote! {}
    };
    let call = if unit.is_async {
        quote! { self.inner.#runner(#arguments).await }
    } else {
        quote! { self.inner.#runner(#arguments) }
    };

    runner_result(unit, &call)
}

fn service_adapter(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let token_binding = token_binding(unit);
    let outcome = runner_outcome(unit);

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

#[must_use]
fn register_servers_definition(
    servers: &[HttpServer],
    has_views: bool,
    activation: SpiffeActivation,
) -> TokenStream {
    if servers.is_empty() {
        return TokenStream::new();
    }

    let body = server_registration(servers, has_views, activation);
    let spiffe_server_parameter = activation
        .server_active
        .then(|| quote! { spiffe_server_config: &::std::sync::Arc<margaret::framework::spiffe_svid::rustls::ServerConfig>, });
    let spiffe_client_parameter = activation.client_active.then(|| {
        quote! {
            spiffe_client_readiness: &margaret::framework::spiffe_svid_client::svid_client_readiness::SvidClientReadiness,
        }
    });
    let too_many_lines = too_many_lines_allow();

    quote! {
        #too_many_lines
        fn register_servers(
            manager: &mut trzcina::ServiceManager,
            matches: &clap::ArgMatches,
            container: &super::container::Container,
            routes: &::std::sync::Arc<super::routes::Routes>,
            #spiffe_server_parameter
            #spiffe_client_parameter
        ) -> ::std::result::Result<
            (),
            margaret::framework::console::command_outcome::CommandOutcome,
        > {
            #body
        }
    }
}

fn register_servers_invocation(
    servers: &[HttpServer],
    activation: SpiffeActivation,
) -> TokenStream {
    if servers.is_empty() {
        return TokenStream::new();
    }

    let spiffe_server_argument = activation
        .server_active
        .then(|| quote! { &spiffe_server_config, });
    let spiffe_client_argument = activation
        .client_active
        .then(|| quote! { &spiffe_client_readiness, });

    quote! {
        if let Err(outcome) = register_servers(
            &mut manager,
            matches,
            container,
            &routes,
            #spiffe_server_argument
            #spiffe_client_argument
        ) {
            return outcome;
        }
    }
}

pub fn render_services(
    plan: &ServicePlan,
    servers: &[HttpServer],
    has_views: bool,
    bindings: &ContainerBindings,
) -> GeneratedModuleTokens {
    let activation = SpiffeActivation {
        client_active: plan.has_spiffe_http_client,
        server_active: serves_spiffe(servers),
    };

    let adapters = plan.units.iter().map(adapter);
    let unit_registrations = plan
        .units
        .iter()
        .map(|unit| registration(unit, bindings, activation.client_active));
    let identity_prelude = svid_identity_prelude(activation);
    let bundle_registration = bundle_registration(activation);
    let served_routes = served_routes(servers);
    let register_servers = register_servers_definition(servers, has_views, activation);
    let server_registration = register_servers_invocation(servers, activation);
    let construction_invocation =
        bindings.serve_invocation(&plan.construction_arguments, &plan.served_roots);
    let construction = quote! {
        let container = match #construction_invocation {
            Ok(container) => container,
            Err(error) => {
                return margaret::framework::console::report_failure::report_failure(error);
            }
        };
        let container = &container;
    };
    let matches_binding =
        if servers.is_empty() && !activation.client_active && !plan.reads_clap_matches {
            quote! { _matches }
        } else {
            quote! { matches }
        };
    let prelude = &plan.prelude;
    let serve_too_many_lines = too_many_lines_allow();

    let tokens = quote! {
        #(#adapters)*

        #register_servers

        #serve_too_many_lines
        pub async fn serve(
            #matches_binding: &clap::ArgMatches,
            cancellation_token: tokio_util::sync::CancellationToken,
        ) -> margaret::framework::console::command_outcome::CommandOutcome {
            #identity_prelude
            #served_routes
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

    GeneratedModuleTokens::new("serve", tokens)
}
