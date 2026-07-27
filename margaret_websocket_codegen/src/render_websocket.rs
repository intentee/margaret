use std::collections::BTreeMap;

use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::build_websocket_plan::build_websocket_plan;
use crate::render_messages::render_messages;
use crate::render_server_routes::render_server_routes;
use crate::render_server_routes::server_console_arguments;
use crate::render_sessions::render_sessions;
use crate::session_plan::SessionPlan;
use crate::websocket_artifacts::WebSocketArtifacts;
use crate::websocket_codegen_error::WebSocketCodegenError;

fn server_arguments(
    sessions_by_server: &BTreeMap<String, Vec<&SessionPlan>>,
    bindings: &ContainerBindings,
) -> Result<BTreeMap<String, Vec<ConsoleArgument>>, WebSocketCodegenError> {
    sessions_by_server
        .iter()
        .map(|(server, sessions)| {
            server_console_arguments(sessions, bindings)
                .map(|arguments| (server.clone(), arguments))
        })
        .collect()
}

fn binding_root(binding: &RequestBinding) -> Option<&CanonicalPath> {
    match binding {
        RequestBinding::AuthenticatedUser { application, .. } => Some(&application.concrete),
        RequestBinding::Bound {
            binder_provider, ..
        } => Some(binder_provider),
        RequestBinding::Injectable { dependency } => Some(&dependency.concrete),
        RequestBinding::AssetBag
        | RequestBinding::CurrentRequest
        | RequestBinding::FormRequest { .. }
        | RequestBinding::Forwarder
        | RequestBinding::Next
        | RequestBinding::PeerSpiffeId
        | RequestBinding::Raw { .. }
        | RequestBinding::Routes
        | RequestBinding::Views => None,
    }
}

fn retained_roots(plan: &crate::websocket_plan::WebSocketPlan) -> Vec<CanonicalPath> {
    let mut roots = std::collections::BTreeSet::new();

    for session_plan in &plan.sessions {
        for layer in &session_plan.session.layers {
            roots.insert(layer.concrete.clone());
        }

        for parameter in &session_plan.session.parameters {
            if let Some(root) = binding_root(&parameter.binding) {
                roots.insert(root.clone());
            }
        }

        for handler in session_plan
            .request_handlers
            .iter()
            .chain(&session_plan.notification_handlers)
        {
            roots.insert(handler.handler_path.clone());
        }
    }

    roots.into_iter().collect()
}
pub fn render_websocket(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    middleware_plans: &[MiddlewarePlan],
    registries: &BindingRegistries,
) -> Result<WebSocketArtifacts, WebSocketCodegenError> {
    let plan = build_websocket_plan(index, bindings, middleware_plans, registries)?;

    let mut sessions_by_server: BTreeMap<String, Vec<&SessionPlan>> = BTreeMap::new();

    for session_plan in &plan.sessions {
        sessions_by_server
            .entry(session_plan.session.server.clone())
            .or_default()
            .push(session_plan);
    }

    let module_declarations = plan.sessions.iter().map(|session_plan| {
        let module = format_ident!("{}", session_plan.session.module_name);

        quote! {
            #[rustfmt::skip]
            pub mod #module;
        }
    });
    let server_routes = sessions_by_server
        .iter()
        .map(|(server, sessions)| render_server_routes(server, sessions, bindings));
    let message_implementations = render_messages(&plan.messages);

    let mut modules = vec![GeneratedModuleTokens::new(
        "websocket",
        quote! {
            #(#module_declarations)*

            #(#server_routes)*

            #message_implementations
        },
    )];

    modules.extend(render_sessions(&plan.sessions, bindings));

    let server_console_arguments = server_arguments(&sessions_by_server, bindings)?;

    Ok(WebSocketArtifacts {
        modules,
        retained_roots: retained_roots(&plan),
        server_console_arguments,
        servers: sessions_by_server.into_keys().collect(),
    })
}
