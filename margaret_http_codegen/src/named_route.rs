use crate::http_route::HttpRoute;
use crate::named_route_shape::NamedRouteShape;

pub(crate) struct NamedRoute<'route> {
    pub(crate) name: &'route str,
    pub(crate) position: usize,
    pub(crate) route: &'route HttpRoute,
    pub(crate) shape: NamedRouteShape,
}
