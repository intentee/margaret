use margaret_route_method::route_method::RouteMethod;

use crate::route_handler::RouteHandler;

pub(crate) struct RoutedHandler {
    pub(crate) handler: RouteHandler,
    pub(crate) method: RouteMethod,
}
