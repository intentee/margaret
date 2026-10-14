use margaret_route_method::route_method::RouteMethod;

use crate::route_handler::RouteHandler;

pub(crate) struct RoutedHandler<TTarget> {
    pub(crate) handler: RouteHandler<TTarget>,
    pub(crate) method: RouteMethod,
}
