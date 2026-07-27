use quote::format_ident;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::render_messages::render_messages;
use crate::render_server_routes::render_server_routes;
use crate::render_sessions::render_sessions;
use crate::web_socket_artifacts::WebSocketArtifacts;
use crate::web_socket_plan::WebSocketPlan;

#[must_use]
pub fn render_websocket(plan: WebSocketPlan, bindings: &ContainerBindings) -> WebSocketArtifacts {
    let module_declarations = plan.sessions.iter().map(|session_plan| {
        let module = format_ident!("{}", session_plan.session.module_name);

        quote! {
            #[rustfmt::skip]
            pub mod #module;
        }
    });
    let route_declarations = plan.sessions_by_server.keys().map(|server| {
        let function = format_ident!("{server}_routes");

        quote! {
            pub mod #function;
            pub use #function::#function;
        }
    });
    let server_route_modules = plan.sessions_by_server.iter().map(|(server, positions)| {
        let sessions = positions
            .iter()
            .map(|position| &plan.sessions[*position])
            .collect::<Vec<_>>();

        GeneratedModuleTokens::new(
            format!("websocket/{server}_routes"),
            render_server_routes(server, &sessions, bindings),
        )
    });
    let message_implementations = render_messages(&plan.messages);

    let mut modules = vec![GeneratedModuleTokens::new(
        "websocket",
        quote! {
            #(#module_declarations)*
            #(#route_declarations)*

            #message_implementations
        },
    )];

    modules.extend(server_route_modules);
    modules.extend(render_sessions(&plan.sessions, bindings));

    WebSocketArtifacts {
        modules,
        retained_roots: plan.retained_roots,
        server_console_arguments: plan.server_console_arguments,
        servers: plan.servers,
    }
}
