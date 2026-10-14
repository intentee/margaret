use std::collections::HashMap;

use crate::route_handler::RouteHandler;

pub(crate) enum RequestRoute<TTarget> {
    Handler {
        handler: RouteHandler<TTarget>,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}
