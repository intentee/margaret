use proc_macro2::Ident;
use syn::Path;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::layer_application::LayerApplication;
use crate::route_parameter::RouteParameter;

pub(crate) struct HttpRoute {
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: Ident,
    pub(crate) path: String,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) route_parameters: Vec<RouteParameter>,
    pub(crate) site_action_guards: Vec<Path>,
}
