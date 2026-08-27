use std::collections::HashMap;
use std::sync::Arc;

use crate::body_intake::BodyIntake;
use crate::handler::Handler;

pub(crate) enum RequestRoute {
    Handler {
        body_intake: BodyIntake,
        handler: Arc<dyn Handler>,
        path_params: HashMap<String, String>,
    },
    MethodNotAllowed,
    NotFound,
}
