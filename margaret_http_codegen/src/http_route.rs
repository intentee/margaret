use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;

pub(crate) struct HttpRoute {
    pub(crate) arguments: Vec<BoundParameter>,
    pub(crate) is_async: bool,
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: String,
    pub(crate) method_name: Ident,
    pub(crate) name: Option<String>,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) server: String,
}
