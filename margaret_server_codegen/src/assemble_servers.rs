use std::collections::BTreeMap;

use crate::http_server::HttpServer;
use crate::server_contribution::ServerContribution;
use crate::server_name::ServerName;
use crate::server_route_source::ServerRouteSource;
use crate::server_transport_policy::ServerTransportPolicy;
use crate::server_transport_requirement::ServerTransportRequirement;

struct AssembledServer {
    serves_web_socket_routes: bool,
    transport_policy: ServerTransportPolicy,
}

impl AssembledServer {
    fn absorb(&mut self, contribution: &ServerContribution) {
        if contribution.routes == ServerRouteSource::WebSocket {
            self.serves_web_socket_routes = true;
        }

        if contribution.transport_requirement == ServerTransportRequirement::VerifiedPeerIdentity {
            self.transport_policy = ServerTransportPolicy::PinnedSpiffeMtls;
        }
    }

    fn negotiable() -> Self {
        Self {
            serves_web_socket_routes: false,
            transport_policy: ServerTransportPolicy::Negotiable,
        }
    }
}

#[must_use]
pub fn assemble_servers(contributions: &[ServerContribution]) -> Vec<HttpServer> {
    let mut assembled: BTreeMap<&ServerName, AssembledServer> = BTreeMap::new();

    for contribution in contributions {
        assembled
            .entry(&contribution.server)
            .or_insert_with(AssembledServer::negotiable)
            .absorb(contribution);
    }

    assembled
        .into_iter()
        .map(|(name, server)| {
            HttpServer::new(
                name.clone(),
                server.serves_web_socket_routes,
                server.transport_policy,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::assemble_servers;
    use crate::server_contribution::ServerContribution;
    use crate::server_name::ServerName;
    use crate::server_route_source::ServerRouteSource;
    use crate::server_transport_policy::ServerTransportPolicy;
    use crate::server_transport_requirement::ServerTransportRequirement;

    fn contribution(
        name: &str,
        routes: ServerRouteSource,
        transport_requirement: ServerTransportRequirement,
    ) -> ServerContribution {
        ServerContribution {
            routes,
            server: ServerName::parse(name.to_string()).expect("the fixture name is snake_case"),
            transport_requirement,
        }
    }

    #[test]
    fn pins_a_server_whose_only_requirement_comes_from_a_web_socket_route() {
        let servers = assemble_servers(&[contribution(
            "gateway",
            ServerRouteSource::WebSocket,
            ServerTransportRequirement::VerifiedPeerIdentity,
        )]);

        assert_eq!(servers.len(), 1);
        assert_eq!(
            servers[0].transport_policy(),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
        assert!(servers[0].serves_web_socket_routes());
    }

    #[test]
    fn pins_a_shared_server_when_only_its_web_socket_route_requires_a_verified_peer() {
        let servers = assemble_servers(&[
            contribution(
                "public",
                ServerRouteSource::Http,
                ServerTransportRequirement::Negotiable,
            ),
            contribution(
                "public",
                ServerRouteSource::WebSocket,
                ServerTransportRequirement::VerifiedPeerIdentity,
            ),
        ]);

        assert_eq!(servers.len(), 1);
        assert_eq!(
            servers[0].transport_policy(),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
        assert!(servers[0].serves_web_socket_routes());
    }

    #[test]
    fn pins_a_shared_server_when_only_its_http_route_requires_a_verified_peer() {
        let servers = assemble_servers(&[
            contribution(
                "public",
                ServerRouteSource::Http,
                ServerTransportRequirement::VerifiedPeerIdentity,
            ),
            contribution(
                "public",
                ServerRouteSource::WebSocket,
                ServerTransportRequirement::Negotiable,
            ),
        ]);

        assert_eq!(servers.len(), 1);
        assert_eq!(
            servers[0].transport_policy(),
            ServerTransportPolicy::PinnedSpiffeMtls
        );
    }

    #[test]
    fn leaves_a_server_negotiable_when_no_route_requires_a_verified_peer() {
        let servers = assemble_servers(&[contribution(
            "public",
            ServerRouteSource::Http,
            ServerTransportRequirement::Negotiable,
        )]);

        assert_eq!(servers.len(), 1);
        assert_eq!(
            servers[0].transport_policy(),
            ServerTransportPolicy::Negotiable
        );
        assert!(!servers[0].serves_web_socket_routes());
    }

    #[test]
    fn orders_distinct_servers_by_name() {
        let servers = assemble_servers(&[
            contribution(
                "public",
                ServerRouteSource::Http,
                ServerTransportRequirement::Negotiable,
            ),
            contribution(
                "internal",
                ServerRouteSource::Http,
                ServerTransportRequirement::Negotiable,
            ),
        ]);

        assert_eq!(
            servers
                .iter()
                .map(|server| server.name().as_str())
                .collect::<Vec<_>>(),
            vec!["internal", "public"]
        );
    }
}
