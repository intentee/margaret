use std::collections::HashMap;

use crate::route_handler::RouteHandler;

pub(crate) enum RequestRoute {
    Handler {
        path_params: HashMap<String, String>,
        route_handler: RouteHandler,
    },
    MethodNotAllowed,
    NotFound,
}
