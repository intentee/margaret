use std::sync::Arc;

use crate::handler::Handler;
use crate::request_cancellation_cooperation::RequestCancellationCooperation;

#[derive(Clone)]
pub(crate) struct RouteHandler {
    pub(crate) cancellation_cooperation: RequestCancellationCooperation,
    pub(crate) handler: Arc<dyn Handler>,
}
