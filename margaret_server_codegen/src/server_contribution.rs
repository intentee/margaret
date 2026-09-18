use crate::server_name::ServerName;
use crate::server_route_source::ServerRouteSource;
use crate::server_transport_requirement::ServerTransportRequirement;

#[derive(Clone, Debug)]
pub struct ServerContribution {
    pub routes: ServerRouteSource,
    pub server: ServerName,
    pub transport_requirement: ServerTransportRequirement,
}
