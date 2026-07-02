use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::layer_application::LayerApplication;
use crate::responder_output::ResponderOutput;
use crate::route_parameter::RouteParameter;

pub(crate) struct HttpRoute {
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: Ident,
    pub(crate) name: Option<String>,
    pub(crate) path: String,
    pub(crate) responder_field: Ident,
    pub(crate) responder_output: ResponderOutput,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) route_parameters: Vec<RouteParameter>,
    pub(crate) server: String,
}

impl HttpRoute {
    pub(crate) fn is_bound(&self) -> bool {
        self.route_parameters
            .iter()
            .any(|route_parameter| route_parameter.binding.is_bound())
    }
}
