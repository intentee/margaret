use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use matchit::InsertError;
use matchit::Router;

use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route::HttpRoute;
use crate::route_group::RouteGroup;

pub(crate) struct ServerRouteGroup {
    matcher: Router<()>,
    paths: BTreeMap<String, RouteGroup>,
}

impl ServerRouteGroup {
    pub(crate) fn new() -> Self {
        Self {
            matcher: Router::new(),
            paths: BTreeMap::new(),
        }
    }

    pub(crate) fn insert(
        &mut self,
        server: &str,
        path: RoutePath,
        method: String,
        route: HttpRoute,
    ) -> Result<(), HttpCodegenError> {
        let responder = route.responder_path.to_string();
        let pattern = path.pattern().to_owned();

        let group = match self.paths.entry(pattern.clone()) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => {
                self.matcher
                    .insert(pattern.clone(), ())
                    .map_err(|source| match source {
                        InsertError::Conflict {
                            with: conflicting_path,
                        } => HttpCodegenError::ConflictingRoutePaths {
                            conflicting_path,
                            path: pattern.clone(),
                            responder: responder.clone(),
                            server: server.to_owned(),
                        },
                        source => HttpCodegenError::InvalidRoutePath {
                            path: pattern.clone(),
                            responder: responder.clone(),
                            source,
                        },
                    })?;

                vacant.insert(RouteGroup::new(path))
            }
        };

        match group.take_method(method.clone(), route) {
            Some(existing) => Err(HttpCodegenError::DuplicateRoute {
                existing_responder: existing.responder_path.to_string(),
                method,
                path: pattern,
                responder,
                server: server.to_owned(),
            }),
            None => Ok(()),
        }
    }

    pub(crate) fn route_groups(&self) -> impl Iterator<Item = &RouteGroup> {
        self.paths.values()
    }

    pub(crate) fn routes(&self) -> impl Iterator<Item = &HttpRoute> {
        self.paths.values().flat_map(RouteGroup::method_routes)
    }
}
