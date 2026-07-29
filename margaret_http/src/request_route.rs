use std::collections::HashMap;
use std::str::Utf8Error;

use crate::route_handler::RouteHandler;

pub(crate) enum RequestRoute {
    Handler {
        path_params: HashMap<String, String>,
        route_handler: RouteHandler,
    },
    MethodNotAllowed,
    NotFound,
    PathParameterNotValidUtf8 {
        parameter: String,
        source: Utf8Error,
    },
}
