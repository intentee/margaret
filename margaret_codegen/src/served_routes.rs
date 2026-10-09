use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_http_codegen::framework_responders::FrameworkResponders;

pub(crate) struct ServedRoutes<'index> {
    pub(crate) declared: DeclaredRoutes<'index>,
    pub(crate) framework: FrameworkResponders,
}
