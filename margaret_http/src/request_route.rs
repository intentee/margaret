use std::collections::HashMap;
use std::str::Utf8Error;
use std::sync::Arc;

use crate::handler::Handler;

pub(crate) enum RequestRoute {
    Handler {
        handler: Arc<dyn Handler>,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
    PathParameterNotValidUtf8 {
        parameter: String,
        source: Utf8Error,
    },
}
