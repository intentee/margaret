use std::collections::BTreeMap;

use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::server_transport_policy::ServerTransportPolicy;
use crate::web_socket_server_requirements::WebSocketServerRequirements;

pub(crate) fn active_servers(
    table: &HttpRouteTable,
    websocket_servers: &BTreeMap<String, WebSocketServerRequirements>,
) -> Vec<HttpServer> {
    let mut policies: BTreeMap<&str, ServerTransportPolicy> = websocket_servers
        .iter()
        .map(|(name, requirements)| (name.as_str(), requirements.transport_policy))
        .collect();

    for route in table.routes() {
        let required = ServerTransportPolicy::required_by(&route.arguments, &route.layers);

        policies
            .entry(route.server.as_str())
            .and_modify(|policy| *policy = policy.combined_with(required))
            .or_insert(required);
    }

    policies
        .into_iter()
        .map(|(name, transport_policy)| HttpServer::new(name.to_string(), transport_policy))
        .collect()
}
