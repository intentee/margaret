use margaret_http::router_error::RouterError;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;

use crate::server_uploads::ServerUploads;

pub struct ServerAssembly {
    pub address_argument: &'static str,
    pub routes: Result<ServerRoutes, RouterError>,
    pub transport: TransportConfig,
    pub uploads: ServerUploads,
}
