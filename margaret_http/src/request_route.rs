use std::collections::HashMap;

use crate::route_handler::RouteHandler;

pub(crate) enum RequestRoute {
    Handler {
        handler: RouteHandler,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}
