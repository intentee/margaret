use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy)]
pub(crate) struct ApplicationFeatures {
    pub(crate) has_authenticated_users: bool,
    pub(crate) has_console: bool,
    pub(crate) has_http: bool,
    pub(crate) has_middleware: bool,
    pub(crate) has_models: bool,
    pub(crate) has_views: bool,
    pub(crate) has_websockets: bool,
    pub(crate) serves: bool,
}

impl ApplicationFeatures {
    pub(crate) fn from_index(index: &AttributeIndex) -> Self {
        let has_authenticated_users =
            index.has_framework_attribute(FrameworkAttribute::InfersAuthenticatedUser);
        let has_commands = index.has_framework_attribute(FrameworkAttribute::ConsoleCommand);
        let has_http = index.has_framework_attribute(FrameworkAttribute::RespondsToHttp);
        let has_middleware =
            index.has_framework_attribute(FrameworkAttribute::HandlesMiddlewareAttribute);
        let has_models = index.has_framework_attribute(FrameworkAttribute::Model);
        let has_services = index.has_framework_attribute(FrameworkAttribute::Service)
            || index.has_framework_attribute(FrameworkAttribute::ScheduledWithTickTimer);
        let has_views = index.has_framework_attribute(FrameworkAttribute::RendersView) && has_http;
        let has_websockets = index.has_framework_attribute(FrameworkAttribute::WebsocketSession);
        let serves = has_http || has_services || has_websockets;
        let has_console = has_commands || serves || has_models;

        Self {
            has_authenticated_users,
            has_console,
            has_http,
            has_middleware,
            has_models,
            has_views,
            has_websockets,
            serves,
        }
    }
}
