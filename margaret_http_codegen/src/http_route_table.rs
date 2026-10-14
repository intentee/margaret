use std::collections::BTreeMap;

use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
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

    pub(crate) fn insert(
        &mut self,
        path: RoutePath,
        route: HttpRoute,
    ) -> Result<(), HttpCodegenError> {
        let method = route.method();
        let server = route.server.clone();

        self.servers
            .entry(server.clone())
            .or_insert_with(ServerRouteGroup::new)
            .insert(&server, path, method, route)
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
