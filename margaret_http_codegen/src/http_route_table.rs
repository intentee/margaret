use std::collections::BTreeMap;

use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::named_route::NamedRoute;
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

    /// # Errors
    ///
    /// Returns `HttpCodegenError::ConflictingForwardTargetBodyIntake` when a responder that
    /// forwards cannot read the request body the way one of its forward targets does.
    pub(crate) fn resolve_forward_target_body_intake(&mut self) -> Result<(), HttpCodegenError> {
        for (server, group) in &mut self.servers {
            group.resolve_forward_target_body_intake(server)?;
        }

        Ok(())
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
