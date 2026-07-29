use std::collections::BTreeMap;
use std::sync::Arc;

use crate::handler::Handler;

pub(crate) enum RequestRoute {
    Handler {
        handler: Arc<dyn Handler>,
        path_params: BTreeMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}
