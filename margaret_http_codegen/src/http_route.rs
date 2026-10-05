use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_route_method::route_method::RouteMethod;

use crate::route_content::RouteContent;

pub(crate) struct HttpRoute {
    pub(crate) arguments: Vec<BoundParameter>,
    pub(crate) content: RouteContent,
    pub(crate) is_async: bool,
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method_name: Ident,
    pub(crate) name: Option<String>,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) server: String,
}

impl HttpRoute {
    pub(crate) fn method(&self) -> RouteMethod {
        self.content.method()
    }
}
