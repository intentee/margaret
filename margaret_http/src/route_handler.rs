use std::sync::Arc;

use crate::content_handler::ContentHandler;
use crate::entered_responders::EnteredResponders;
use crate::head_handler::HeadHandler;

pub(crate) enum RouteHandler<TTarget> {
    Content(Arc<dyn ContentHandler<TTarget>>),
    Head(Arc<dyn HeadHandler<TTarget>>),
}

impl<TTarget> Clone for RouteHandler<TTarget> {
    fn clone(&self) -> Self {
        match self {
            Self::Content(handler) => Self::Content(Arc::clone(handler)),
            Self::Head(handler) => Self::Head(Arc::clone(handler)),
        }
    }
}

impl<TTarget> RouteHandler<TTarget> {
    pub(crate) fn entered(&self) -> EnteredResponders {
        match self {
            Self::Content(handler) => EnteredResponders::starting_at(handler),
            Self::Head(handler) => EnteredResponders::starting_at(handler),
        }
    }
}
