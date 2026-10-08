use margaret_request_binding_codegen::route_model::RouteModel;
use margaret_tag_codegen::tag_pool::TagPool;

pub(crate) struct BindingDeclarations<'declarations, 'tags> {
    pub(crate) route_models: &'declarations [RouteModel],
    pub(crate) tags: &'tags TagPool<'tags>,
}
