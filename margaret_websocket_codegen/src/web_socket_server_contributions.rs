use margaret_server_codegen::server_contribution::ServerContribution;
use margaret_server_codegen::server_route_source::ServerRouteSource;
use margaret_server_codegen::server_transport_requirement::ServerTransportRequirement;

use crate::session_plan::SessionPlan;

pub(crate) fn web_socket_server_contributions(sessions: &[SessionPlan]) -> Vec<ServerContribution> {
    sessions
        .iter()
        .map(|session_plan| ServerContribution {
            routes: ServerRouteSource::WebSocket,
            server: session_plan.session.server.clone(),
            transport_requirement: ServerTransportRequirement::for_route(
                &session_plan.session.parameters,
                &session_plan.session.layers,
            ),
        })
        .collect()
}
