use crate::http_server::HttpServer;
use crate::server_transport_policy::ServerTransportPolicy;

pub fn serves_spiffe(servers: &[HttpServer]) -> bool {
    servers.iter().any(|server| {
        matches!(
            server.transport_policy(),
            ServerTransportPolicy::PinnedSpiffeMtls
        )
    })
}
