use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::observes_cancellation::observes_cancellation;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub(crate) struct HttpRoute {
    pub(crate) arguments: Vec<BoundParameter>,
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: String,
    pub(crate) method_name: Ident,
    pub(crate) name: Option<String>,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) server: String,
}

impl HttpRoute {
    pub(crate) fn forwards(&self) -> bool {
        self.arguments
            .iter()
            .any(|parameter| matches!(parameter.binding, RequestBinding::Forwarder))
    }

    pub(crate) fn is_forwardable(&self) -> bool {
        self.name.is_some() && self.method == "GET"
    }

    pub(crate) fn observes_cancellation(&self) -> bool {
        observes_cancellation(&self.arguments)
            || self.layers.iter().any(|layer| layer.observes_cancellation)
    }
}
