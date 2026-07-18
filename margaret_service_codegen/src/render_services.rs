use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::ensure_unique::ensure_unique;
use margaret_console_argument_codegen::required_flag_read::required_flag_read;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::rendered_services::RenderedServices;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

fn transport_expression(server: &HttpServer, spiffe_secured: bool) -> TokenStream {
    if !spiffe_secured {
        return quote! { margaret_http::transport_config::TransportConfig::Plain };
    }

    match server.transport_policy() {
        ServerTransportPolicy::PinnedSpiffeMtls => quote! {
            margaret_http::transport_config::TransportConfig::MutualTls {
                server_config: spiffe_server_config.clone(),
            }
        },
        ServerTransportPolicy::Negotiable => {
            let transport_argument = server.transport_argument();

            quote! {
                match matches.get_one::<String>(#transport_argument).map(String::as_str) {
                    Some("spiffe_mtls") => margaret_http::transport_config::TransportConfig::MutualTls {
                        server_config: spiffe_server_config.clone(),
                    },
                    _ => margaret_http::transport_config::TransportConfig::Plain,
                }
            }
        }
    }
}

fn server_manager_setup(servers: &[HttpServer]) -> TokenStream {
    if servers.is_empty() {
        return quote! {
            let mut manager = trzcina::ServiceManager::default();
        };
    }

    let spiffe_secured = serves_spiffe(servers);
    let spiffe_prelude = spiffe_secured.then(|| {
        let spiffe_trust_domain =
            required_flag_read(&quote! { String }, "spiffe-trust-domain", &quote! { value.clone() });
        let spire_agent_addr =
            required_flag_read(&quote! { String }, "spire-agent-addr", &quote! { value.clone() });

        quote! {
            margaret_spiffe_svid::install_default_crypto_provider::install_default_crypto_provider();

            let spiffe_bundle = margaret_spiffe_svid_server::SvidServerBundle::new(
                margaret_spiffe_svid::SvidServiceBundleParams {
                    spiffe_trust_domain: #spiffe_trust_domain,
                    spire_agent_addr: #spire_agent_addr,
                },
            );
            let spiffe_server_config = ::std::sync::Arc::new(spiffe_bundle.server_config());
        }
    });

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

    let assemblies = servers.iter().map(|server| {
        let function_name = server.function_name();
        let name = server.name();
        let address_argument = server.address_argument();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let transport = transport_expression(server, spiffe_secured);

        quote! {
            margaret_service::server_assembly::ServerAssembly {
                address_argument: #address_argument,
                name: #name,
                routes: super::http::#function_name::#function_name(container, &routes).await,
                transport: #transport,
                upload_dir_argument: #upload_dir_argument,
                uploads_argument: #uploads_argument,
            }
        }
    });
    let assemblies = vec_literal_tokens(assemblies);

    let bundle = if spiffe_secured {
        quote! { ::std::option::Option::Some(spiffe_bundle) }
    } else {
        quote! { ::std::option::Option::<margaret_service::no_bundle::NoBundle>::None }
    };

    quote! {
        #spiffe_prelude

        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );
        let servers = #assemblies;

        let mut manager = match margaret_service::serve_application::serve_application(
            matches,
            servers,
            #bundle,
        )
        .await
        {
            Ok(manager) => manager,
            Err(outcome) => return outcome,
        };
    }
}

fn argument_field_idents(unit: &ServiceUnit) -> Vec<Ident> {
    (0..unit.arguments.len())
        .map(|position| format_ident!("argument_{position}"))
        .collect()
}

fn missed_tick_behavior(behavior: &Option<Path>) -> TokenStream {
    match behavior {
        Some(behavior) => quote! { #behavior },
        None => quote! { tokio::time::MissedTickBehavior::default() },
    }
}

fn adapter(unit: &ServiceUnit) -> TokenStream {
    if unit.arguments.is_empty() {
        adapter_without_arguments(unit)
    } else {
        adapter_with_arguments(unit)
    }
}

fn adapter_without_arguments(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let token_binding = if unit.uses_token() {
        quote! { cancellation_token }
    } else {
        quote! { _cancellation_token }
    };
    let body = adapter_body(unit);
    let tick_runner_impl = tick_runner_impl(unit);

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
                #body
            }
        }

        #tick_runner_impl
    }
}

fn adapter_body(unit: &ServiceUnit) -> TokenStream {
    match &unit.kind {
        ServiceKind::Service => {
            let runner = format_ident!("{}", unit.runner);
            let call = if unit.takes_token {
                quote! { self.inner.#runner(cancellation_token).await? }
            } else {
                quote! { self.inner.#runner().await? }
            };

            quote! {
                #call;

                Ok(())
            }
        }
        ServiceKind::Ticker { behavior, interval } => {
            let missed_tick_behavior = missed_tick_behavior(behavior);

            quote! {
                margaret_service::run_scheduled_service::run_scheduled_service(
                    #interval,
                    #missed_tick_behavior,
                    cancellation_token,
                    self.inner,
                )
                .await
            }
        }
    }
}

fn tick_runner_impl(unit: &ServiceUnit) -> TokenStream {
    let ServiceKind::Ticker { .. } = &unit.kind else {
        return quote! {};
    };

    let concrete = path_tokens(&unit.concrete_path);
    let runner = format_ident!("{}", unit.runner);
    let (token_binding, call) = if unit.takes_token {
        (
            quote! { cancellation_token },
            quote! { self.#runner(cancellation_token).await },
        )
    } else {
        (
            quote! { _cancellation_token },
            quote! { self.#runner().await },
        )
    };

    quote! {
        #[async_trait::async_trait]
        impl margaret_service::tick_runner::TickRunner for #concrete {
            async fn tick(
                &self,
                #token_binding: tokio_util::sync::CancellationToken,
            ) -> anyhow::Result<()> {
                #call.map_err(anyhow::Error::from)
            }
        }
    }
}

fn adapter_with_arguments(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let fields = unit
        .arguments
        .iter()
        .enumerate()
        .map(|(position, argument)| {
            let field = format_ident!("argument_{position}");
            let field_type = argument.field_type();

            quote! { #field: #field_type, }
        });
    let body = adapter_with_arguments_body(unit);
    let scheduled_runner = scheduled_argument_runner_impl(unit);

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
            #(#fields)*
        }

        #[async_trait::async_trait]
        impl trzcina::Service for #name {
            async fn run(
                self: Box<Self>,
                cancellation_token: tokio_util::sync::CancellationToken,
            ) -> anyhow::Result<()> {
                #body
            }
        }

        #scheduled_runner
    }
}

fn adapter_with_arguments_body(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let runner = format_ident!("{}", unit.runner);
    let fields = argument_field_idents(unit);
    let destructure = quote! { let #name { inner, #(#fields),* } = *self; };

    match &unit.kind {
        ServiceKind::Service => {
            let call = if unit.takes_token {
                quote! { inner.#runner(#(#fields,)* cancellation_token).await? }
            } else {
                quote! { inner.#runner(#(#fields),*).await? }
            };

            quote! {
                #destructure

                #call;

                Ok(())
            }
        }
        ServiceKind::Ticker { behavior, interval } => {
            let missed_tick_behavior = missed_tick_behavior(behavior);
            let argument = &fields[0];

            quote! {
                #destructure

                margaret_service::run_scheduled_service_with_argument::run_scheduled_service_with_argument(
                    #interval,
                    #missed_tick_behavior,
                    cancellation_token,
                    inner,
                    #argument,
                )
                .await
            }
        }
    }
}

fn scheduled_argument_runner_impl(unit: &ServiceUnit) -> TokenStream {
    let ServiceKind::Ticker { .. } = &unit.kind else {
        return quote! {};
    };

    let concrete = path_tokens(&unit.concrete_path);
    let runner = format_ident!("{}", unit.runner);
    let argument_type = unit.arguments[0].field_type();

    quote! {
        #[async_trait::async_trait]
        impl margaret_service::scheduled_argument_runner::ScheduledArgumentRunner<#argument_type>
            for #concrete
        {
            async fn run_scheduled_tick(&self, argument: #argument_type) -> anyhow::Result<()> {
                self.#runner(argument).await.map_err(anyhow::Error::from)
            }
        }
    }
}

fn registration(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let accessor = format_ident!("{}", unit.field_name);

    if unit.arguments.is_empty() {
        return quote! {
            manager.register_service(#name { inner: container.#accessor().await });
        };
    }

    let field_assignments = unit
        .arguments
        .iter()
        .enumerate()
        .map(|(position, argument)| {
            let field = format_ident!("argument_{position}");
            let value = argument_value(argument);

            quote! { #field: #value, }
        });

    quote! {
        manager.register_service(#name {
            inner: container.#accessor().await,
            #(#field_assignments)*
        });
    }
}

fn adapter_ident(unit: &ServiceUnit) -> Ident {
    format_ident!("{}", unit.type_name)
}

fn serve_arguments(units: &[ServiceUnit]) -> Result<Vec<ConsoleArgument>, ServiceCodegenError> {
    let declarations: Vec<(String, Vec<ConsoleArgument>)> = units
        .iter()
        .map(|unit| (unit.concrete_path.to_string(), unit.arguments.clone()))
        .collect();

    ensure_unique(&declarations)?;

    Ok(units
        .iter()
        .flat_map(|unit| unit.arguments.clone())
        .collect())
}

pub fn render_services(
    index: &AttributeIndex,
    servers: &[HttpServer],
) -> Result<RenderedServices, ServiceCodegenError> {
    let units = service_units(index)?;
    let serve_arguments = serve_arguments(&units)?;
    let adapters = units.iter().map(adapter);
    let registrations = units.iter().map(registration);
    let manager_setup = server_manager_setup(servers);
    let matches_binding = if servers.is_empty() && serve_arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };

    let tokens = quote! {
        #(#adapters)*

        pub async fn serve(
            container: &super::container::Container,
            #matches_binding: &clap::ArgMatches,
            cancellation_token: tokio_util::sync::CancellationToken,
        ) -> margaret_console::command_outcome::CommandOutcome {
            #manager_setup

            #(#registrations)*

            margaret_service::run::run(
                manager,
                cancellation_token,
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    Ok(RenderedServices {
        module: GeneratedModuleTokens::new("serve", tokens),
        serve_arguments,
    })
}
