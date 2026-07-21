use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub(crate) struct WebSocketSession {
    pub(crate) layers: Vec<LayerApplication>,
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

    pub(crate) fn references_routes(&self) -> bool {
        self.injects_routes()
            || self
                .layers
                .iter()
                .any(|application| application.injects_routes)
    }
}
