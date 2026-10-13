use std::collections::BTreeMap;

use margaret_request_binding_codegen::content_binding::ContentBinding;

use crate::application_responder::ApplicationResponder;
use crate::http_route_table::HttpRouteTable;
use crate::http_server::HttpServer;
use crate::route_content::RouteContent;
use crate::route_responder::RouteResponder;
use crate::server_transport_policy::ServerTransportPolicy;
use crate::server_uploads::ServerUploads;
use crate::web_socket_server_requirements::WebSocketServerRequirements;

fn uploads_of(table: &HttpRouteTable, server: &str) -> ServerUploads {
    if table.routes().any(|route| {
        route.server == server
            && matches!(
                route.responder,
                RouteResponder::Application(ApplicationResponder {
                    content: RouteContent::Read {
                        binding: ContentBinding::MultipartFiles { .. }
                            | ContentBinding::MultipartFieldsAndFiles { .. },
                        ..
                    },
                    ..
                })
            )
    }) {
        ServerUploads::Accepted
    } else {
        ServerUploads::Refused
    }
}

pub(crate) fn active_servers(
    table: &HttpRouteTable,
    websocket_servers: &BTreeMap<String, WebSocketServerRequirements>,
) -> Vec<HttpServer> {
    let mut policies: BTreeMap<&str, ServerTransportPolicy> = websocket_servers
        .iter()
        .map(|(name, requirements)| (name.as_str(), requirements.transport_policy))
        .collect();

    for route in table.routes() {
        let required = ServerTransportPolicy::required_by(route.arguments(), &route.layers);

        policies
            .entry(route.server.as_str())
            .and_modify(|policy| *policy = policy.combined_with(required))
            .or_insert(required);
    }

    policies
        .into_iter()
        .map(|(name, transport_policy)| {
            HttpServer::new(name.to_string(), transport_policy, uploads_of(table, name))
        })
        .collect()
}
