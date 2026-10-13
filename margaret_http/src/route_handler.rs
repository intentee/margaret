use std::sync::Arc;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;

#[derive(Clone)]
pub(crate) enum RouteHandler {
    Content(Arc<dyn ContentHandler>),
    Head(Arc<dyn HeadHandler>),
}
