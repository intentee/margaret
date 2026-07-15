use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::layer_application::LayerApplication;
use crate::responder_argument::ResponderArgument;

pub(crate) struct HttpRoute {
    pub(crate) arguments: Vec<ResponderArgument>,
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: String,
    pub(crate) name: Option<String>,
    pub(crate) responder_field: Ident,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) server: String,
}
