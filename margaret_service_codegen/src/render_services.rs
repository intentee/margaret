use std::collections::BTreeMap;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::vec_literal_tokens::vec_literal_tokens;
use margaret_console_argument_codegen::argument_value::argument_value;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::required_flag_read::required_flag_read;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::jwks_client_endpoint::JwksClientEndpoint;
use crate::jwks_client_verifier_path::jwks_client_verifier_path;
use crate::jwks_server_publication_path::jwks_server_publication_path;
use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

fn framework_service_registrations(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
) -> Result<Vec<TokenStream>, ServiceCodegenError> {
    let mut registrations = Vec::new();

    if bindings.provides(&jwks_server_publication_path()) {
        let field = format_ident!("{}", jwks_server_publication_path().field_name());

        registrations.push(quote! {
            let jwks_publication = container.#field().await;

            manager.register_service(
                margaret_jwks_roller_server::jwks_roller_service::JwksRollerService::new(
                    jwks_publication.jwks_document_holder(),
                    jwks_publication.jwks_secret_holder(),
                    ::std::sync::Arc::new(
                        margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage,
                    ),
                ),
            );
        });
    }

    if let Some(endpoint) = JwksClientEndpoint::resolve(index, bindings)? {
        let verifier_field = format_ident!("{}", jwks_client_verifier_path().field_name());
        let endpoint_field = format_ident!("{}", endpoint.field_name);
        let endpoint_woven = bindings.console_weaves_owned(&endpoint.console_arguments);

        registrations.push(quote! {
            let public_jwks_verifier = container.#verifier_field().await;

            manager.register_service(
                margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService::new(
                    public_jwks_verifier.public_jwks_holder(),
                    container.#endpoint_field(#(#endpoint_woven),*).await,
                    margaret_jwks_client::default_http_client::default_http_client(),
                ),
            );
        });
    }

    Ok(registrations)
}

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

fn server_manager_setup(
    servers: &[HttpServer],
    has_views: bool,
    registers_services: bool,
    bindings: &ContainerBindings,
    server_console_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    views_console_arguments: &[ConsoleArgument],
) -> TokenStream {
    let manager_binding = if registers_services {
        quote! { mut manager }
    } else {
        quote! { manager }
    };

    if servers.is_empty() {
        return quote! {
            let #manager_binding = trzcina::ServiceManager::default();
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

            let spiffe_server = margaret_spiffe_svid_server::SvidServer::new(
                #spiffe_trust_domain,
                #spire_agent_addr,
            );
            let spiffe_server_config = ::std::sync::Arc::new(spiffe_server.server_config());
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

    let empty_arguments: Vec<ConsoleArgument> = Vec::new();
    let views_argument = has_views.then(|| quote! { , &views });
    let assemblies = servers.iter().map(|server| {
        let function_name = server.function_name();
        let name = server.name();
        let address_argument = server.address_argument();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let transport = transport_expression(server, spiffe_secured);
        let server_borrows = bindings.console_borrows(
            server_console_arguments
                .get(server.name())
                .unwrap_or(&empty_arguments),
        );

        quote! {
            margaret_service::server_assembly::ServerAssembly {
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

    let bundle_services_binding = if spiffe_secured {
        quote! { mut bundle_services }
    } else {
        quote! { bundle_services }
    };
    let bundle_services = spiffe_secured.then(|| {
        quote! {
            bundle_services.extend(spiffe_server.into_services());
        }
    });
    let views_setup = has_views.then(|| {
        let views_borrows = bindings.console_borrows(views_console_arguments);

        quote! {
            let views = ::std::sync::Arc::new(super::views::build::build(container, #(#views_borrows)*).await);
        }
    });

    quote! {
        #spiffe_prelude

        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );
        #views_setup
        let servers = #assemblies;

        let #bundle_services_binding: ::std::vec::Vec<
            ::std::boxed::Box<dyn trzcina::Service>,
        > = ::std::vec::Vec::new();

        #bundle_services

        let #manager_binding = match margaret_service::serve_application::serve_application(
            matches,
            servers,
            margaret_service::resolved_services::ResolvedServices {
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

fn serve_prelude(serve_arguments: &[ConsoleArgument], bindings: &ContainerBindings) -> TokenStream {
    let resolutions = serve_arguments.iter().map(|argument| {
        let ident = console_argument_ident(bindings.console_slot(argument.name()));
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
    serve_arguments: &[ConsoleArgument],
    server_console_arguments: &BTreeMap<String, Vec<ConsoleArgument>>,
    views_console_arguments: &[ConsoleArgument],
) -> Result<GeneratedModuleTokens, ServiceCodegenError> {
    let units = service_units(index)?;
    let framework_registrations = framework_service_registrations(index, bindings)?;
    let adapters = units.iter().map(adapter);
    let registrations = units.iter().map(|unit| registration(unit, bindings));
    let manager_setup = server_manager_setup(
        servers,
        has_views,
        !units.is_empty() || !framework_registrations.is_empty(),
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

    let tokens = quote! {
        #(#adapters)*

        pub async fn serve(
            container: &super::container::Container,
            #matches_binding: &clap::ArgMatches,
            cancellation_token: tokio_util::sync::CancellationToken,
        ) -> margaret_console::command_outcome::CommandOutcome {
            #prelude
            #manager_setup

            #(#framework_registrations)*

            #(#registrations)*

            margaret_service::run::run(
                manager,
                cancellation_token,
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    Ok(GeneratedModuleTokens::new("serve", tokens))
}
