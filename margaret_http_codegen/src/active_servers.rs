use std::collections::BTreeMap;

use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::server_transport_policy::ServerTransportPolicy;

pub(crate) fn active_servers(table: &HttpRouteTable) -> Vec<HttpServer> {
    let mut policies: BTreeMap<&str, ServerTransportPolicy> = BTreeMap::new();

    for route in table.routes() {
        policies.insert(
            route.server.as_str(),
            ServerTransportPolicy::PinnedSpiffeMtls,
        );
    }

    policies
        .into_iter()
        .map(|(name, transport_policy)| HttpServer::new(name.to_string(), transport_policy))
        .collect()
}
