use std::ptr;

use proc_macro2::Ident;

use crate::http_route::HttpRoute;
use crate::http_server::HttpServer;
use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::server_forwarding::ServerForwarding;

pub(crate) struct ServerLayout<'plan> {
    pub(crate) field: Ident,
    pub(crate) forwarding: ServerForwarding,
    pub(crate) named: Vec<NamedRoute<'plan>>,
    pub(crate) origin: Ident,
    pub(crate) paths: Ident,
    pub(crate) server: &'plan HttpServer,
    pub(crate) struct_ident: Ident,
}

impl ServerLayout<'_> {
    pub(crate) fn forward_target_field(&self, route: &HttpRoute) -> Option<&Ident> {
        self.named
            .iter()
            .filter(|named| ptr::eq(named.route, route))
            .find_map(|named| match &named.shape {
                NamedRouteShape::ForwardableLiteral { field, .. }
                | NamedRouteShape::ForwardableParameterized { field, .. } => Some(field),
                NamedRouteShape::ReferenceLiteral { .. }
                | NamedRouteShape::ReferenceParameterized { .. } => None,
            })
    }
}
