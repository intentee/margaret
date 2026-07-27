use margaret_http::router_error::RouterError;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;

pub struct ServerAssembly {
    pub address_argument: &'static str,
    pub name: &'static str,
    pub routes: Result<ServerRoutes, RouterError>,
    pub transport: TransportConfig,
    pub upload_dir_argument: &'static str,
    pub uploads_argument: &'static str,
}
