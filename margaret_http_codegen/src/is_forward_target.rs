use crate::http_route::HttpRoute;

pub(crate) fn is_forward_target(route: &HttpRoute) -> bool {
    route.name.is_some() && route.method == "GET"
}
