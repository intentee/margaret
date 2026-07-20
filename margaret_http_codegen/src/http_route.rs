use crate::layer_application::LayerApplication;
use crate::responder_argument::ResponderArgument;
use crate::route_handler::RouteHandler;

pub(crate) struct HttpRoute {
    pub(crate) arguments: Vec<ResponderArgument>,
    pub(crate) handler: RouteHandler,
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method: String,
    pub(crate) name: Option<String>,
    pub(crate) server: String,
}
