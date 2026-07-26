use margaret_route_parameter_codegen::route_path::RoutePath;

use crate::http_route::HttpRoute;

pub(crate) struct ResponderRoute<'table> {
    pub(crate) path: &'table RoutePath,
    pub(crate) route: &'table HttpRoute,
}
