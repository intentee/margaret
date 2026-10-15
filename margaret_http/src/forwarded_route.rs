use std::collections::HashMap;
use std::sync::Arc;

use crate::head_handler::HeadHandler;
use crate::route_props::RouteProps;
use crate::url_segment::UrlSegment;

pub struct ForwardedRoute<TTarget> {
    pub(crate) handler: Arc<dyn HeadHandler<TTarget>>,
    pub(crate) name: &'static str,
    pub(crate) path_params: HashMap<String, String>,
}

impl<TTarget> ForwardedRoute<TTarget> {
    #[must_use]
    pub fn new<TProps: RouteProps>(handler: &Arc<dyn HeadHandler<TTarget>>, props: TProps) -> Self {
        Self {
            handler: Arc::clone(handler),
            name: TProps::NAME,
            path_params: props
                .into_segments()
                .into_iter()
                .filter_map(|segment| match segment {
                    UrlSegment::CatchAllParameter { name, value, .. }
                    | UrlSegment::Parameter { name, value, .. } => Some((name.to_string(), value)),
                    UrlSegment::Literal { .. } => None,
                })
                .collect(),
        }
    }
}
