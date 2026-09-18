use crate::http_server::HttpServer;
use crate::server_transport_policy::ServerTransportPolicy;

#[must_use]
pub fn serves_spiffe(servers: &[HttpServer]) -> bool {
    servers.iter().any(|server| {
        matches!(
            server.transport_policy(),
            ServerTransportPolicy::PinnedSpiffeMtls
        )
    })
}

#[cfg(test)]
mod tests {
    use super::serves_spiffe;
    use crate::assemble_servers::assemble_servers;
    use crate::server_contribution::ServerContribution;
    use crate::server_name::ServerName;
    use crate::server_route_source::ServerRouteSource;
    use crate::server_transport_requirement::ServerTransportRequirement;

    fn servers_requiring(transport_requirement: ServerTransportRequirement) -> Vec<ServerContribution> {
        vec![ServerContribution {
            routes: ServerRouteSource::Http,
            server: ServerName::parse("public".to_string())
                .expect("the fixture name is snake_case"),
            transport_requirement,
        }]
    }

    #[test]
    fn reports_a_pinned_server() {
        assert!(serves_spiffe(&assemble_servers(&servers_requiring(
            ServerTransportRequirement::VerifiedPeerIdentity
        ))));
    }

    #[test]
    fn reports_no_pinned_server_when_every_server_negotiates() {
        assert!(!serves_spiffe(&assemble_servers(&servers_requiring(
            ServerTransportRequirement::Negotiable
        ))));
    }
}
