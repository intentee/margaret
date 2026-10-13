use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_route_method::route_method::RouteMethod;

use crate::application_responder::ApplicationResponder;
use crate::framework_route::FrameworkRoute;
use crate::route_content::RouteContent;
use crate::route_handling::RouteHandling;
use crate::route_responder::RouteResponder;

fn handling_of<TBinding>(content: &RouteContent<TBinding>) -> RouteHandling {
    match content {
        RouteContent::Read { method, .. } => RouteHandling::Content(*method),
        RouteContent::Unread { method } => RouteHandling::Head(*method),
    }
}

pub(crate) struct HttpRoute {
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) name: Option<String>,
    pub(crate) responder: RouteResponder,
    pub(crate) responder_path: CanonicalPath,
    pub(crate) server: String,
}

impl HttpRoute {
    pub(crate) fn arguments(&self) -> &[BoundParameter] {
        match &self.responder {
            RouteResponder::Application(ApplicationResponder { arguments, .. }) => arguments,
            RouteResponder::Framework(_) => &[],
        }
    }

    pub(crate) fn handling(&self) -> RouteHandling {
        match &self.responder {
            RouteResponder::Application(ApplicationResponder { content, .. }) => {
                handling_of(content)
            }
            RouteResponder::Framework(FrameworkRoute { content, .. }) => handling_of(content),
        }
    }

    pub(crate) fn method(&self) -> RouteMethod {
        match self.handling() {
            RouteHandling::Content(method) => method.route_method(),
            RouteHandling::Head(method) => method,
        }
    }
}
