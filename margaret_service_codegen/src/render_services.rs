use heck::ToUpperCamelCase;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::path_tokens::path_tokens;
use margaret_http_codegen::http_server::HttpServer;

use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_units::service_units;

fn server_registrations(servers: &[HttpServer]) -> TokenStream {
    let registrations = servers.iter().map(|server| {
        let function_name = server.function_name();
        let address_argument = server.address_argument();
        let address_variable = format_ident!("address_{}", server.name());

        quote! {
            let #address_variable = matches
                .get_one::<String>(#address_argument)
                .expect("a required console argument is present")
                .clone();

            manager.register_service(
                margaret_service::server_service::ServerService::new(
                    super::http::#function_name(container).await,
                    #address_variable,
                ),
            );
        }
    });

    quote! { #(#registrations)* }
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
        impl margaret_service::Service for #name {
            async fn run(
                self: Box<Self>,
                #token_binding: margaret_service::CancellationToken,
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
    let suffix = match unit.kind {
        ServiceKind::Service => "Service",
        ServiceKind::Ticker { .. } => "Ticker",
    };

    format_ident!("{}{}", unit.field_name.to_upper_camel_case(), suffix)
}

pub fn render_services(
    index: &AttributeIndex,
    servers: &[HttpServer],
) -> Result<String, ServiceCodegenError> {
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
        use super::container::Container;

        #(#adapters)*

        pub async fn serve(
            container: &Container,
            #matches_binding: &clap::ArgMatches,
            cancellation_token: margaret_service::CancellationToken,
        ) -> margaret_console::command_outcome::CommandOutcome {
            let mut manager = margaret_service::ServiceManager::default();

            #server_registration
            #(#registrations)*

            margaret_service::run::run(
                manager,
                cancellation_token,
                margaret_service::ServiceShutdownOptions::default(),
            )
            .await
        }
    };

    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    Ok(prettyplease::unparse(&file))
}
