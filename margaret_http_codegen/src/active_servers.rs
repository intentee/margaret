use std::collections::BTreeMap;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::server_transport_policy::ServerTransportPolicy;

fn requires_peer_spiffe_id(route: &HttpRoute) -> bool {
    route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, ResponderArgumentBinding::PeerSpiffeId))
}

pub(crate) fn active_servers(table: &HttpRouteTable) -> Vec<HttpServer> {
    let mut policies: BTreeMap<&str, ServerTransportPolicy> = BTreeMap::new();

    for route in table.routes() {
        let policy = policies
            .entry(route.server.as_str())
            .or_insert(ServerTransportPolicy::Negotiable);

        if requires_peer_spiffe_id(route) {
            *policy = ServerTransportPolicy::PinnedSpiffeMtls;
        }
    }

    policies
        .into_iter()
        .map(|(name, transport_policy)| HttpServer::new(name.to_string(), transport_policy))
        .collect()
}
