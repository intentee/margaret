use margaret_http::matchit::InsertError;
use margaret_http::server_routes::ServerRoutes;
use margaret_http::transport_config::TransportConfig;

pub struct ServerAssembly {
    pub address_argument: &'static str,
    pub body_limit_argument: &'static str,
    pub name: &'static str,
    pub routes: Result<ServerRoutes, InsertError>,
    pub transport: TransportConfig,
    pub upload_dir_argument: &'static str,
    pub uploads_argument: &'static str,
}
