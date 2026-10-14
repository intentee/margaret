use std::sync::Arc;

use crate::forward_scope::ForwardScope;
use crate::route_entry::RouteEntry;
use crate::route_position::RoutePosition;
use crate::router::Router;
use crate::router_error::RouterError;
use crate::server_routes::ServerRoutes;

pub struct ServerRouteEntries<TTarget: ForwardScope> {
    entries: Vec<RouteEntry<TTarget>>,
    matcher: Arc<matchit::Router<RoutePosition>>,
    paths: Vec<&'static str>,
    targets: TTarget::Targets,
}

impl<TTarget: ForwardScope> ServerRouteEntries<TTarget> {
    pub(crate) fn new<const COUNT: usize>(
        matcher: Arc<matchit::Router<RoutePosition>>,
        paths: [&'static str; COUNT],
        entries: [RouteEntry<TTarget>; COUNT],
        targets: TTarget::Targets,
    ) -> Self {
        Self {
            entries: Vec::from(entries),
            matcher,
            paths: Vec::from(paths),
            targets,
        }
    }

    /// # Errors
    ///
    /// Returns `RouterError::DuplicateMethod` when the entry of a path answers one method twice.
    pub fn build(self) -> Result<ServerRoutes<TTarget>, RouterError> {
        Ok(ServerRoutes {
            router: Router::build(self.matcher, self.paths, self.entries)?,
            targets: self.targets,
        })
    }
}
