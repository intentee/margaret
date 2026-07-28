use std::sync::Arc;

use crate::handler::Handler;
use crate::request_cancellation_cooperation::RequestCancellationCooperation;
use crate::route_handler::RouteHandler;

pub struct MethodHandler {
    pub(crate) method: &'static str,
    pub(crate) route_handler: RouteHandler,
}

impl MethodHandler {
    #[must_use]
    pub fn new(
        method: &'static str,
        handler: Arc<dyn Handler>,
        cancellation_cooperation: RequestCancellationCooperation,
    ) -> Self {
        Self {
            method,
            route_handler: RouteHandler {
                cancellation_cooperation,
                handler,
            },
        }
    }
}
