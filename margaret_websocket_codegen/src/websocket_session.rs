use margaret_attributes::canonical_path::CanonicalPath;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub(crate) struct WebSocketSession {
    pub(crate) module_name: String,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) path: String,
    pub(crate) server: String,
    pub(crate) session_path: CanonicalPath,
}

impl WebSocketSession {
    pub(crate) fn injects_routes(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Routes))
    }

    pub(crate) fn injects_views(&self) -> bool {
        self.parameters
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Views))
    }
}
