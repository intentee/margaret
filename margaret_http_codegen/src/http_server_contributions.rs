use margaret_server_codegen::server_contribution::ServerContribution;
use margaret_server_codegen::server_route_source::ServerRouteSource;
use margaret_server_codegen::server_transport_requirement::ServerTransportRequirement;

use crate::http_route_table::HttpRouteTable;

pub(crate) fn http_server_contributions(table: &HttpRouteTable) -> Vec<ServerContribution> {
    table
        .routes()
        .map(|route| ServerContribution {
            routes: ServerRouteSource::Http,
            server: route.server.clone(),
            transport_requirement: ServerTransportRequirement::for_route(
                &route.arguments,
                &route.layers,
            ),
        })
        .collect()
}
