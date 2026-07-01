use proc_macro2::Ident;
use syn::Path;

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
    pub(crate) site_action_guards: Vec<Path>,
}

impl HttpRoute {
    pub(crate) fn needs_user(&self) -> bool {
        !self.site_action_guards.is_empty()
            || self.route_parameters.iter().any(|route_parameter| {
                route_parameter.binding.is_actor() || route_parameter.binding.is_authorizing()
            })
    }

    pub(crate) fn is_bound(&self) -> bool {
        self.route_parameters
            .iter()
            .any(|route_parameter| route_parameter.binding.is_bound())
    }

    pub(crate) fn is_authorized(&self) -> bool {
        self.route_parameters
            .iter()
            .any(|route_parameter| route_parameter.binding.is_authorizing())
    }
}
