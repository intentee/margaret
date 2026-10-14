use std::sync::Arc;

use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;

pub(crate) enum MethodHandlerKind<TTarget> {
    Content {
        handler: Arc<dyn ContentHandler<TTarget>>,
        method: ContentMethod,
    },
    Head {
        handler: Arc<dyn HeadHandler<TTarget>>,
        method: RouteMethod,
    },
}
