use std::sync::Arc;

use crate::addressable_route::AddressableRoute;
use crate::forward_scope::ForwardScope;
use crate::route_entry::RouteEntry;
use crate::route_position::RoutePosition;
use crate::router_error::RouterError;
use crate::server_route_entries::ServerRouteEntries;

pub struct ServerPaths<const COUNT: usize> {
    matcher: Arc<matchit::Router<RoutePosition>>,
    paths: [&'static str; COUNT],
}

impl<const COUNT: usize> ServerPaths<COUNT> {
    /// # Errors
    ///
    /// Returns `RouterError::Path` when the router cannot route a path or two paths conflict.
    pub fn build(paths: [&'static str; COUNT]) -> Result<Self, RouterError> {
        let mut matcher = matchit::Router::new();

        for (index, path) in paths.iter().enumerate() {
            matcher.insert(*path, RoutePosition::new(index))?;
        }

        Ok(Self {
            matcher: Arc::new(matcher),
            paths,
        })
    }

    #[must_use]
    pub fn addressable<TProps, TReference>(
        &self,
        origin: &Arc<str>,
        position: RoutePosition,
        reference: fn(String) -> TReference,
    ) -> AddressableRoute<TProps, TReference> {
        AddressableRoute::new(
            Arc::clone(&self.matcher),
            Arc::clone(origin),
            position,
            reference,
        )
    }

    #[must_use]
    pub fn routes<TTarget: ForwardScope>(
        &self,
        entries: [RouteEntry<TTarget>; COUNT],
        targets: TTarget::Targets,
    ) -> ServerRouteEntries<TTarget> {
        ServerRouteEntries::new(Arc::clone(&self.matcher), self.paths, entries, targets)
    }
}
