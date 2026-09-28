use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::server_transport_policy::ServerTransportPolicy;
use margaret_http_codegen::web_socket_server_requirements::WebSocketServerRequirements;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_roots::binding_roots;

use crate::build_websocket_plan::build_websocket_plan;
use crate::built_websocket_plan::BuiltWebSocketPlan;
use crate::server_serve_inputs::server_serve_inputs;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;

fn retained_roots(sessions: &[SessionPlan]) -> Vec<CanonicalPath> {
    let mut roots = BTreeSet::new();

    for session_plan in sessions {
        for layer in &session_plan.session.layers {
            roots.insert(layer.concrete.clone());
        }

        for parameter in &session_plan.session.parameters {
            roots.extend(binding_roots(&parameter.binding).into_iter().cloned());
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
    pub(crate) servers: BTreeMap<String, WebSocketServerRequirements>,
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
        middleware_plans: &MiddlewarePlans,
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

        let servers = sessions_by_server
            .iter()
            .map(|(server, positions)| {
                let sessions = positions
                    .iter()
                    .map(|position| &sessions[*position])
                    .collect::<Vec<_>>();
                let transport_policy = sessions
                    .iter()
                    .map(|session_plan| {
                        ServerTransportPolicy::required_by(
                            &session_plan.session.parameters,
                            &session_plan.session.layers,
                        )
                    })
                    .fold(
                        ServerTransportPolicy::Negotiable,
                        ServerTransportPolicy::combined_with,
                    );

                server_serve_inputs(&sessions, bindings).map(|serve_inputs| {
                    (
                        server.clone(),
                        WebSocketServerRequirements {
                            serve_inputs,
                            transport_policy,
                        },
                    )
                })
            })
            .collect::<Result<_, _>>()?;
        let retained_roots = retained_roots(&sessions);

        Ok(Self {
            messages,
            retained_roots,
            servers,
            sessions,
            sessions_by_server,
        })
    }
}
