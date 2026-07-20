use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_route::HttpRoute;

pub(crate) struct NamedRoute<'route> {
    pub(crate) name: &'route str,
    pub(crate) path: &'route RoutePath,
    pub(crate) route: &'route HttpRoute,
}
