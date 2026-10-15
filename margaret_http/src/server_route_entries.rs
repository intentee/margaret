use std::sync::Arc;

use crate::route_entry::RouteEntry;
use crate::route_position::RoutePosition;
use crate::router::Router;
use crate::router_error::RouterError;

pub struct ServerRouteEntries {
    entries: Vec<RouteEntry>,
    matcher: Arc<matchit::Router<RoutePosition>>,
    paths: Vec<&'static str>,
}

impl ServerRouteEntries {
    pub(crate) fn new<const COUNT: usize>(
        matcher: Arc<matchit::Router<RoutePosition>>,
        paths: [&'static str; COUNT],
        entries: [RouteEntry; COUNT],
    ) -> Self {
        Self {
            entries: Vec::from(entries),
            matcher,
            paths: Vec::from(paths),
        }
    }

    /// # Errors
    ///
    /// Returns `RouterError::DuplicateMethod` when the entry of a path answers one method twice.
    pub fn build(self) -> Result<Router, RouterError> {
        Router::build(self.matcher, self.paths, self.entries)
    }
}
