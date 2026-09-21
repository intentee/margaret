use std::collections::BTreeMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_root::binding_root;
use margaret_server_codegen::server_contribution::ServerContribution;
use margaret_server_codegen::server_name::ServerName;

use crate::build_websocket_plan::build_websocket_plan;
use crate::built_websocket_plan::BuiltWebSocketPlan;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;
use crate::web_socket_server_contributions::web_socket_server_contributions;

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
    pub(crate) server_contributions: Vec<ServerContribution>,
    pub(crate) sessions: Vec<SessionPlan>,
    pub(crate) sessions_by_server: BTreeMap<ServerName, Vec<usize>>,
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
        let mut sessions_by_server: BTreeMap<ServerName, Vec<usize>> = BTreeMap::new();

        for (position, session_plan) in sessions.iter().enumerate() {
            sessions_by_server
                .entry(session_plan.session.server.clone())
                .or_default()
                .push(position);
        }

        let server_contributions = web_socket_server_contributions(&sessions);
        let retained_roots = retained_roots(&sessions);

        Ok(Self {
            messages,
            retained_roots,
            server_contributions,
            sessions,
            sessions_by_server,
        })
    }
}
