use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_http_codegen::framework_responders::FrameworkResponders;
use margaret_route_parameter_codegen::route_path_registry::RoutePathRegistry;

pub(crate) struct ServedRoutes<'index> {
    pub(crate) declared: DeclaredRoutes<'index>,
    pub(crate) framework: FrameworkResponders,
    pub(crate) paths: RoutePathRegistry,
}
