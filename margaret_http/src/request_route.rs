use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::Handler;

pub(crate) enum RequestRoute {
    Handler {
        handler: Arc<dyn Handler>,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}
