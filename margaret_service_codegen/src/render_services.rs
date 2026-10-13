use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::routes_ident::routes_ident;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_container::container_bindings::ContainerBindings;
use margaret_container::reported_construction::reported_construction;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;
use margaret_http_codegen::http_server::HttpServer;

use crate::runner_outcome::RunnerOutcome;
use crate::serve_invocation_field::ServeInvocationField;
use crate::serve_invocation_fields::serve_invocation_fields;
use crate::server_bindings::ServerBindings;
use crate::service_kind::ServiceKind;
use crate::service_plan::ServicePlan;
use crate::service_unit::ServiceUnit;
use crate::service_unit_origin::ServiceUnitOrigin;
use crate::spiffe_activation::SpiffeActivation;

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

fn adapter(unit: &ServiceUnit) -> TokenStream {
    match &unit.kind {
        ServiceKind::Service => service_adapter(unit),
        ServiceKind::Ticker { behavior, interval } => {
            ticker_adapter(unit, behavior.as_ref(), interval)
        }
    }
}

fn runner_result(unit: &ServiceUnit, call: &TokenStream) -> TokenStream {
    match unit.origin {
        ServiceUnitOrigin::Framework {
            outcome: RunnerOutcome::Fallible,
        } => quote! {
            #call.map_err(margaret::framework::anyhow::Error::from)
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
    interval: &CanonicalPath,
) -> TokenStream {
    let name = adapter_ident(unit);
    let concrete = path_tokens(&unit.concrete_path);
    let interval = path_tokens(interval);
    let missed_tick_behavior_method = missed_tick_behavior_method(behavior);
    let token_binding = token_binding(unit);
    let outcome = runner_outcome(unit);

    quote! {
        struct #name {
            inner: std::sync::Arc<#concrete>,
        }

        #[async_trait::async_trait]
        impl trzcina::Ticker for #name {
            async fn handle_tick(
                &mut self,
                #token_binding: tokio_util::sync::CancellationToken,
                _tick_context: trzcina::TickContext,
            ) -> margaret::framework::anyhow::Result<()> {
                #outcome
            }

            #missed_tick_behavior_method

            fn tick_interval(&self) -> std::time::Duration {
                #interval
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

fn register_servers_definition(
    servers: &[HttpServer],
    has_views: bool,
    activation: SpiffeActivation,
) -> TokenStream {
    if servers.is_empty() {
        return TokenStream::new();
    }

    let listeners = servers
        .iter()
        .map(|server| ServerBindings::of(server).listener)
        .collect::<Vec<Ident>>();
    let count = Literal::usize_unsuffixed(listeners.len());
    let views_setup = has_views.then(|| {
        quote! {
            let views = ::std::sync::Arc::new(super::views::build::build(container));
        }
    });
    let views_argument = has_views.then(|| quote! { , &views });
    let registrations = servers.iter().zip(&listeners).map(|(server, listener)| {
        let function_name = server.function_name();

        gated_registration(
            &quote! {
                margaret::framework::service::server_service::ServerService::new(
                    #listener,
                    super::http::#function_name::#function_name(container, routes #views_argument),
                )
            },
            activation.client_active,
        )
    });
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
            [#(#listeners),*]: [margaret::framework::service::server_listener::ServerListener; #count],
            container: &super::container::Container,
            routes: &::std::sync::Arc<super::routes::Routes>,
            #spiffe_client_parameter
        ) {
            #views_setup
            #(#registrations)*
        }
    }
}

fn server_registration(servers: &[HttpServer], activation: SpiffeActivation) -> TokenStream {
    if servers.is_empty() {
        return TokenStream::new();
    }

    let routes = routes_ident();
    let spiffe_client_argument = activation
        .client_active
        .then(|| quote! { &spiffe_client_readiness, });

    quote! {
        register_servers(&mut manager, listeners, container, &#routes, #spiffe_client_argument);
    }
}

#[must_use]
pub fn render_services(
    plan: &ServicePlan,
    servers: &[HttpServer],
    has_views: bool,
    bindings: &ContainerBindings,
) -> Vec<GeneratedModuleTokens> {
    let activation = SpiffeActivation::of(plan, servers);
    let fields = serve_invocation_fields(plan, servers, bindings);
    let declarations = fields.iter().map(ServeInvocationField::declaration);
    let names = fields.iter().map(ServeInvocationField::name);
    let adapters = plan.units.iter().map(adapter);
    let unit_registrations = plan
        .units
        .iter()
        .map(|unit| registration(unit, bindings, activation.client_active));
    let register_servers = register_servers_definition(servers, has_views, activation);
    let server_registration = server_registration(servers, activation);
    let construction_arguments = if plan.construction_arguments.is_empty() {
        TokenStream::new()
    } else {
        quote! { arguments }
    };
    let constructed_container = reported_construction(
        &bindings.serve_invocation(&construction_arguments, &plan.served_roots),
        bindings.serve_outcome(&plan.served_roots),
    );
    let client_readiness = activation.client_active.then(|| {
        quote! { let spiffe_client_readiness = spiffe_bundle.client_readiness(); }
    });
    let bundle_registration = activation.any().then(|| {
        quote! {
            margaret::framework::spiffe_svid::register_services::register_services(
                &mut manager,
                spiffe_bundle.into_services(),
            );
        }
    });
    let serve_too_many_lines = too_many_lines_allow();
    let invocation_type = quote! { self::serve_invocation::ServeInvocation };

    vec![
        GeneratedModuleTokens::new(
            ModulePath::root("serve"),
            quote! {
                pub mod serve_invocation;

                #(#adapters)*

                #register_servers

                #serve_too_many_lines
                pub async fn serve(
                    #invocation_type { #(#names),* }: #invocation_type,
                    cancellation_token: tokio_util::sync::CancellationToken,
                ) -> margaret::framework::console::command_outcome::CommandOutcome {
                    #client_readiness
                    let container = #constructed_container;
                    let container = &container;
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
            },
        ),
        GeneratedModuleTokens::new(
            ModulePath::root("serve").child("serve_invocation"),
            quote! {
                pub struct ServeInvocation {
                    #(#declarations)*
                }
            },
        ),
    ]
}
