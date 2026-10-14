use std::sync::Arc;

use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::method_handler_kind::MethodHandlerKind;
use crate::route_handler::RouteHandler;
use crate::routed_handler::RoutedHandler;

pub struct MethodHandler<TTarget> {
    kind: MethodHandlerKind<TTarget>,
}

impl<TTarget> MethodHandler<TTarget> {
    #[must_use]
    pub fn content(method: ContentMethod, handler: Arc<dyn ContentHandler<TTarget>>) -> Self {
        Self {
            kind: MethodHandlerKind::Content { handler, method },
        }
    }

    #[must_use]
    pub fn head(method: RouteMethod, handler: Arc<dyn HeadHandler<TTarget>>) -> Self {
        Self {
            kind: MethodHandlerKind::Head { handler, method },
        }
    }

    pub(crate) fn into_routed(self) -> RoutedHandler<TTarget> {
        match self.kind {
            MethodHandlerKind::Content { handler, method } => RoutedHandler {
                handler: RouteHandler::Content(handler),
                method: method.route_method(),
            },
            MethodHandlerKind::Head { handler, method } => RoutedHandler {
                handler: RouteHandler::Head(handler),
                method,
            },
        }
    }
}
