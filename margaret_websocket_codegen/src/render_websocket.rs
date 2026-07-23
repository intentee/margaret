use std::collections::BTreeMap;

use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;

use crate::render_messages::render_messages;
use crate::render_server_routes::render_server_routes;
use crate::render_server_routes::server_console_arguments;
use crate::render_sessions::render_sessions;
use crate::session_plan::SessionPlan;
use crate::websocket_artifacts::WebSocketArtifacts;
use crate::websocket_codegen_error::WebSocketCodegenError;
use crate::websocket_plan::websocket_plan;

pub fn render_websocket(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    has_views: bool,
    middleware_plans: &[MiddlewarePlan],
) -> Result<WebSocketArtifacts, WebSocketCodegenError> {
    let plan = websocket_plan(index, bindings, middleware_plans)?;

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
        .map(|(server, sessions)| render_server_routes(server, sessions, has_views, bindings));
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

    let server_console_arguments = sessions_by_server
        .iter()
        .map(|(server, sessions)| (server.clone(), server_console_arguments(sessions, bindings)))
        .collect();

    Ok(WebSocketArtifacts {
        modules,
        server_console_arguments,
        servers: sessions_by_server.into_keys().collect(),
    })
}
