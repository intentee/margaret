use crate::http_route::HttpRoute;
use crate::route_path::RoutePath;

pub(crate) struct NamedRoute<'route> {
    pub(crate) name: &'route str,
    pub(crate) path: &'route RoutePath,
    pub(crate) route: &'route HttpRoute,
}
