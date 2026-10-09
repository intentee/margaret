use crate::application_responder::ApplicationResponder;
use crate::framework_route::FrameworkRoute;

pub(crate) enum RouteResponder {
    Application(ApplicationResponder),
    Framework(FrameworkRoute),
}
