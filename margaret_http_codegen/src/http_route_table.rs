use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::named_route::NamedRoute;
use crate::responder_route::ResponderRoute;
use crate::route_group::RouteGroup;
use crate::server_route_group::ServerRouteGroup;

pub(crate) struct HttpRouteTable {
    servers: BTreeMap<String, ServerRouteGroup>,
}

impl HttpRouteTable {
    pub(crate) fn new() -> Self {
        Self {
            servers: BTreeMap::new(),
        }
    }

    pub(crate) fn find_by_responder(&self, responder: &CanonicalPath) -> Option<ResponderRoute<'_>> {
        self.servers
            .values()
            .flat_map(ServerRouteGroup::route_groups)
            .find_map(|group| {
                group
                    .method_routes()
                    .find(|route| &route.responder_path == responder)
                    .map(|route| ResponderRoute {
                        path: group.path(),
                        route,
                    })
            })
    }

    pub(crate) fn insert(
        &mut self,
        path: RoutePath,
        route: HttpRoute,
    ) -> Result<(), HttpCodegenError> {
        let method = route.method.clone();
        let server = route.server.clone();

        self.servers
            .entry(server.clone())
            .or_insert_with(ServerRouteGroup::new)
            .insert(&server, path, method, route)
    }

    pub(crate) fn named_routes(&self, server: &str) -> Vec<NamedRoute<'_>> {
        let mut named: Vec<NamedRoute<'_>> = self
            .route_groups(server)
            .flat_map(|group| {
                group.method_routes().filter_map(|route| {
                    route.name.as_deref().map(|name| NamedRoute {
                        name,
                        path: group.path(),
                        route,
                    })
                })
            })
            .collect();

        named.sort_by(|first, second| first.name.cmp(second.name));

        named
    }

    pub(crate) fn route_groups(&self, server: &str) -> impl Iterator<Item = &RouteGroup> {
        self.servers
            .get(server)
            .into_iter()
            .flat_map(ServerRouteGroup::route_groups)
    }

    pub(crate) fn routes(&self) -> impl Iterator<Item = &HttpRoute> {
        self.servers.values().flat_map(ServerRouteGroup::routes)
    }
}
