use std::sync::Arc;

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
    pub fn routes(&self, entries: [RouteEntry; COUNT]) -> ServerRouteEntries {
        ServerRouteEntries::new(Arc::clone(&self.matcher), self.paths, entries)
    }

    pub(crate) fn resolves_to(&self, path: &str, position: RoutePosition) -> bool {
        self.matcher
            .at(path)
            .is_ok_and(|matched| *matched.value == position)
    }
}

#[cfg(test)]
mod tests {
    use super::ServerPaths;
    use crate::route_position::RoutePosition;

    #[test]
    fn resolves_a_path_to_the_position_of_its_pattern_so_builders_recognize_their_route() {
        let paths =
            ServerPaths::build(["/articles/new", "/articles/{article}"]).expect("the paths build");

        assert!(paths.resolves_to("/articles/rust", RoutePosition::new(1)));
    }

    #[test]
    fn resolves_a_shadowed_path_to_the_static_route_exactly_like_the_server_does() {
        let paths =
            ServerPaths::build(["/articles/new", "/articles/{article}"]).expect("the paths build");

        assert!(!paths.resolves_to("/articles/new", RoutePosition::new(1)));
    }
}
