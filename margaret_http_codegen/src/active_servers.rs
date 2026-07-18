use std::collections::BTreeMap;

use crate::http_route::HttpRoute;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::responder_argument_binding::ResponderArgumentBinding;
use crate::server_cookie_policy::ServerCookiePolicy;
use crate::server_transport_policy::ServerTransportPolicy;

struct ServerPolicies {
    cookie_policy: ServerCookiePolicy,
    transport_policy: ServerTransportPolicy,
}

fn injects_cookie_jar(route: &HttpRoute) -> bool {
    route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, ResponderArgumentBinding::CookieJar))
        || route.layers.iter().any(|layer| layer.injects_cookie_jar)
}

fn requires_peer_spiffe_id(route: &HttpRoute) -> bool {
    route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, ResponderArgumentBinding::PeerSpiffeId))
}

pub(crate) fn active_servers(table: &HttpRouteTable) -> Vec<HttpServer> {
    let mut policies: BTreeMap<&str, ServerPolicies> = BTreeMap::new();

    for route in table.routes() {
        let policies = policies
            .entry(route.server.as_str())
            .or_insert(ServerPolicies {
                cookie_policy: ServerCookiePolicy::Cookieless,
                transport_policy: ServerTransportPolicy::Negotiable,
            });

        if injects_cookie_jar(route) {
            policies.cookie_policy = ServerCookiePolicy::Cookied;
        }

        if requires_peer_spiffe_id(route) {
            policies.transport_policy = ServerTransportPolicy::PinnedSpiffeMtls;
        }
    }

    policies
        .into_iter()
        .map(
            |(
                name,
                ServerPolicies {
                    cookie_policy,
                    transport_policy,
                },
            )| HttpServer::new(name.to_string(), cookie_policy, transport_policy),
        )
        .collect()
}
