use margaret_attributes::canonical_path::CanonicalPath;
use margaret_route_method::route_method::RouteMethod;

use crate::served_session_endpoint::ServedSessionEndpoint;

pub struct DeclaredSessionEndpointRoute {
    pub endpoint: ServedSessionEndpoint,
    pub method: RouteMethod,
    pub route: CanonicalPath,
}
