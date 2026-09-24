use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::injects_peer_spiffe_id::injects_peer_spiffe_id;
use margaret_request_binding_codegen::injects_routes::injects_routes;
use margaret_request_binding_codegen::injects_views::injects_views;

pub(crate) struct MiddlewareInjections {
    pub(crate) peer_spiffe_id: bool,
    pub(crate) routes: bool,
    pub(crate) views: bool,
}

impl MiddlewareInjections {
    pub(crate) fn new(parameters: &[BoundParameter]) -> Self {
        Self {
            peer_spiffe_id: injects_peer_spiffe_id(parameters),
            routes: injects_routes(parameters),
            views: injects_views(parameters),
        }
    }
}
