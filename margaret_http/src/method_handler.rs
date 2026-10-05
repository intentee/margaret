use std::sync::Arc;

use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::method_handler_kind::MethodHandlerKind;
use crate::named_handler::NamedHandler;
use crate::route_handler::RouteHandler;
use crate::routed_handler::RoutedHandler;

pub struct MethodHandler {
    kind: MethodHandlerKind,
}

impl MethodHandler {
    #[must_use]
    pub fn content(method: ContentMethod, handler: Arc<dyn ContentHandler>) -> Self {
        Self {
            kind: MethodHandlerKind::Content { handler, method },
        }
    }

    #[must_use]
    pub fn forwardable(name: &'static str, handler: Arc<dyn HeadHandler>) -> Self {
        Self {
            kind: MethodHandlerKind::Forwardable { handler, name },
        }
    }

    #[must_use]
    pub fn head(method: RouteMethod, handler: Arc<dyn HeadHandler>) -> Self {
        Self {
            kind: MethodHandlerKind::Head { handler, method },
        }
    }

    pub(crate) fn forward_target(&self) -> Option<NamedHandler> {
        match &self.kind {
            MethodHandlerKind::Forwardable { handler, name } => {
                Some(NamedHandler::new(name, Arc::clone(handler)))
            }
            MethodHandlerKind::Content { .. } | MethodHandlerKind::Head { .. } => None,
        }
    }

    pub(crate) fn into_routed(self) -> RoutedHandler {
        match self.kind {
            MethodHandlerKind::Content { handler, method } => RoutedHandler {
                handler: RouteHandler::Content(handler),
                method: method.route_method(),
            },
            MethodHandlerKind::Forwardable { handler, .. } => RoutedHandler {
                handler: RouteHandler::Head(handler),
                method: RouteMethod::Get,
            },
            MethodHandlerKind::Head { handler, method } => RoutedHandler {
                handler: RouteHandler::Head(handler),
                method,
            },
        }
    }
}
