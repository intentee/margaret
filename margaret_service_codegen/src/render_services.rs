use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

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

fn server_registrations(servers: &[HttpServer]) -> TokenStream {
    if servers.is_empty() {
        return quote! {};
    }

    let spiffe_secured = serves_spiffe(servers);
    let spiffe_prelude = spiffe_secured.then(|| {
        quote! {
            margaret_spiffe_svid_manager::install_default_crypto_provider::install_default_crypto_provider();

            let spiffe_bundle = margaret_spiffe_svid_manager::SvidServiceBundle::new(
                margaret_spiffe_svid_manager::SvidServiceBundleParams {
                    spiffe_trust_domain: match matches.get_one::<String>("spiffe-trust-domain") {
                        Some(value) => value.clone(),
                        None => return margaret_console::command_outcome::CommandOutcome::Failed,
                    },
                    spire_agent_addr: match matches.get_one::<String>("spire-agent-addr") {
                        Some(value) => value.clone(),
                        None => return margaret_console::command_outcome::CommandOutcome::Failed,
                    },
                },
            );
            let spiffe_server_config = ::std::sync::Arc::new(spiffe_bundle.server_config());
        }
    });
    let spiffe_bundle_registration = spiffe_secured.then(|| {
        quote! {
            if let Err(error) = manager.register_bundle(spiffe_bundle).await {
                return margaret_console::report_failure::report_failure(error);
            }
        }
    });

    let origins = servers.iter().map(|server| {
        let name = server.name();
        let address_argument = server.address_argument();
        let url_argument = server.url_argument();
        let address_variable = format_ident!("address_{}", name);
        let origin_variable = format_ident!("origin_{}", name);

        quote! {
            let #address_variable = match matches.get_one::<String>(#address_argument) {
                Some(value) => value.clone(),
                None => return margaret_console::command_outcome::CommandOutcome::Failed,
            };
            let #origin_variable: ::std::sync::Arc<str> = matches
                .get_one::<String>(#url_argument)
                .cloned()
                .unwrap_or_else(|| ::std::format!("http://{}", #address_variable))
                .into();
        }
    });

    let origin_arguments = servers.iter().map(|server| {
        let origin_variable = format_ident!("origin_{}", server.name());

        quote! { #origin_variable.clone() }
    });

    let builds = servers.iter().map(|server| {
        let function_name = server.function_name();
        let name = server.name();
        let uploads_argument = server.uploads_argument();
        let upload_dir_argument = server.upload_dir_argument();
        let routes_variable = format_ident!("routes_{}", name);
        let address_variable = format_ident!("address_{}", name);
        let origin_variable = format_ident!("origin_{}", name);
        let upload_config_variable = format_ident!("upload_config_{}", name);
        let transport_variable = format_ident!("transport_{}", name);
        let forward_targets_variable = format_ident!("forward_targets_{}", name);
        let transport = transport_expression(server, spiffe_secured);

        quote! {
            let #routes_variable = super::http::#function_name::#function_name(container, &routes).await;
            let #upload_config_variable = if matches.get_flag(#uploads_argument) {
                margaret_http::upload_config::UploadConfig::enabled(
                    matches
                        .get_one::<String>(#upload_dir_argument)
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(std::env::temp_dir),
                )
            } else {
                margaret_http::upload_config::UploadConfig::Disabled
            };
            let #transport_variable = #transport;
            let #forward_targets_variable = ::std::sync::Arc::new(
                margaret_http::forward_targets::ForwardTargets::new(#routes_variable.named_handlers),
            );

            server_models.push(margaret_http::server::Server::new(
                #name,
                #address_variable,
                #origin_variable,
                #transport_variable,
                #upload_config_variable,
                margaret_http::body_limit::BodyLimit::default(),
                #routes_variable.router,
            ));
        }
    });

    let registrations = servers.iter().map(|server| {
        let name = server.name();
        let forward_targets_variable = format_ident!("forward_targets_{}", name);

        quote! {
            manager.register_service(
                margaret_service::server_service::ServerService::new(
                    server_registry.clone(),
                    #forward_targets_variable,
                    #name,
                ),
            );
        }
    });

    quote! {
        let mut server_models = Vec::new();

        #spiffe_prelude

        #(#origins)*

        let routes = ::std::sync::Arc::new(
            super::routes::Routes::from_origins(#(#origin_arguments),*),
        );

        #(#builds)*

        let server_registry = ::std::sync::Arc::new(
            margaret_http::server_registry::ServerRegistry::new(server_models),
        );

        #spiffe_bundle_registration

        #(#registrations)*
    }
}

fn adapter(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let token_binding = if unit.uses_token() {
        quote! { cancellation_token }
    } else {
        quote! { _cancellation_token }
    };
    let body = adapter_body(unit);

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
    }
}

fn adapter_body(unit: &ServiceUnit) -> TokenStream {
    let runner = format_ident!("{}", unit.runner);

    match &unit.kind {
        ServiceKind::Service => {
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
            let set_behavior = match behavior {
                Some(behavior) => quote! { interval.set_missed_tick_behavior(#behavior); },
                None => quote! {},
            };
            let tick = if unit.takes_token {
                quote! { self.inner.#runner(cancellation_token.clone()).await? }
            } else {
                quote! { self.inner.#runner().await? }
            };

            quote! {
                let mut interval = tokio::time::interval(#interval);

                #set_behavior

                loop {
                    tokio::select! {
                        _ = cancellation_token.cancelled() => return Ok(()),
                        _ = interval.tick() => {
                            #tick;
                        }
                    }
                }
            }
        }
    }
}

fn registration(unit: &ServiceUnit) -> TokenStream {
    let name = adapter_ident(unit);
    let accessor = format_ident!("{}", unit.field_name);
    let inner = quote! { container.#accessor().await };

    quote! {
        manager.register_service(#name { inner: #inner });
    }
}

fn adapter_ident(unit: &ServiceUnit) -> Ident {
    format_ident!("{}", unit.type_name)
}

pub fn render_services(
    index: &AttributeIndex,
    servers: &[HttpServer],
) -> Result<GeneratedModuleTokens, ServiceCodegenError> {
    let units = service_units(index)?;
    let adapters = units.iter().map(adapter);
    let registrations = units.iter().map(registration);
    let server_registration = server_registrations(servers);
    let matches_binding = if servers.is_empty() {
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
            let mut manager = trzcina::ServiceManager::default();

            #server_registration
            #(#registrations)*

            margaret_service::run::run(
                manager,
                cancellation_token,
                trzcina::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    Ok(GeneratedModuleTokens::new("services", tokens))
}
