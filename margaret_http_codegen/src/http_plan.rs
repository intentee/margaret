use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::binding_roots::binding_roots;

use crate::active_servers::active_servers;
use crate::declared_routes::DeclaredRoutes;
use crate::framework_responders::FrameworkResponders;
use crate::framework_route::FrameworkRoute;
use crate::http_codegen_error::HttpCodegenError;
use crate::http_route_table::HttpRouteTable;
use crate::http_routes::http_routes;
use crate::http_server::HttpServer;
use crate::route_responder::RouteResponder;
use crate::web_socket_server_requirements::WebSocketServerRequirements;

fn retained_roots(table: &HttpRouteTable) -> Vec<CanonicalPath> {
    let mut roots = BTreeSet::new();

    for route in table.routes() {
        match &route.responder {
            RouteResponder::Application(_) => roots.insert(route.responder_path.clone()),
            RouteResponder::Framework(FrameworkRoute { handler, .. }) => {
                roots.insert(handler.concrete.clone())
            }
        };

        for layer in &route.layers {
            roots.insert(layer.concrete.clone());
        }

        for parameter in route.arguments() {
            roots.extend(binding_roots(&parameter.binding).into_iter().cloned());
        }
    }

    roots.into_iter().collect()
}

pub struct HttpPlan {
    pub(crate) has_views: bool,
    pub(crate) retained_roots: Vec<CanonicalPath>,
    pub(crate) servers: Vec<HttpServer>,
    pub(crate) table: HttpRouteTable,
    pub(crate) websocket_servers: BTreeSet<String>,
}

impl HttpPlan {
    /// # Errors
    ///
    /// Returns `HttpCodegenError` propagated from the work it performs.
    pub fn build(
        index: &AttributeIndex,
        routes: DeclaredRoutes,
        framework_responders: FrameworkResponders,
        has_views: bool,
        websocket_servers: &BTreeMap<String, WebSocketServerRequirements>,
        middleware_plans: &MiddlewarePlans,
        registries: &BindingRegistries,
    ) -> Result<Self, HttpCodegenError> {
        let mut table = http_routes(
            index,
            routes,
            framework_responders,
            middleware_plans,
            registries,
        )?;

        for (server, requirements) in websocket_servers {
            for route in &requirements.sessions {
                table.reserve_web_socket(server, route)?;
            }
        }

        let servers = active_servers(&table, websocket_servers);
        let retained_roots = retained_roots(&table);

        Ok(Self {
            has_views,
            retained_roots,
            servers,
            table,
            websocket_servers: websocket_servers.keys().cloned().collect(),
        })
    }
}
