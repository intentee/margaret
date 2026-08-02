use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_root::binding_root;

use crate::build_websocket_plan::build_websocket_plan;
use crate::built_websocket_plan::BuiltWebSocketPlan;
use crate::server_console_arguments::server_console_arguments;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;

fn retained_roots(sessions: &[SessionPlan]) -> Vec<CanonicalPath> {
    let mut roots = std::collections::BTreeSet::new();

    for session_plan in sessions {
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

pub struct WebSocketPlan {
    pub(crate) messages: Vec<WebSocketMessage>,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) server_console_arguments: BTreeMap<String, Vec<ConsoleArgument>>,
    pub(crate) servers: Vec<String>,
    pub(crate) sessions: Vec<SessionPlan>,
    pub(crate) sessions_by_server: BTreeMap<String, Vec<usize>>,
}

impl WebSocketPlan {
    /// # Errors
    ///
    /// Returns `WebSocketCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        bindings: &ContainerBindings,
        middleware_plans: &[MiddlewarePlan],
        registries: &BindingRegistries,
    ) -> Result<Self, WebSocketCodegenError> {
        let BuiltWebSocketPlan { messages, sessions } =
            build_websocket_plan(index, bindings, middleware_plans, registries)?;
        let mut sessions_by_server: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        for (position, session_plan) in sessions.iter().enumerate() {
            sessions_by_server
                .entry(session_plan.session.server.clone())
                .or_default()
                .push(position);
        }

        let server_console_arguments = sessions_by_server
            .iter()
            .map(|(server, positions)| {
                let sessions = positions
                    .iter()
                    .map(|position| &sessions[*position])
                    .collect::<Vec<_>>();

                server_console_arguments(&sessions, bindings)
                    .map(|arguments| (server.clone(), arguments))
            })
            .collect::<Result<_, _>>()?;
        let servers = sessions_by_server.keys().cloned().collect();
        let retained_roots = retained_roots(&sessions);

        Ok(Self {
            messages,
            retained_roots,
            server_console_arguments,
            servers,
            sessions,
            sessions_by_server,
        })
    }
}
