use crate::canonical_path::CanonicalPath;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrameworkAttribute {
    AuthenticatedUser,
    BuildForSession,
    Column,
    ConsoleArgument,
    ConsoleCommand,
    Constructor,
    EnvironmentVariable,
    ForeignKey,
    FormRequest,
    HandlesMiddlewareAttribute,
    Index,
    InferFromRequest,
    InfersAuthenticatedUser,
    JwksSecretStore,
    Middleware,
    Model,
    PrimaryKey,
    Process,
    ProvidesJwksEndpoint,
    ProvidesRouteParameter,
    RendersView,
    RespondsToHttp,
    RouteParameter,
    RouteParameterValue,
    ScheduledWithTickTimer,
    Service,
    Singleton,
    SpiffeHttpClient,
    SpiffeWebsocketClient,
    Unique,
    WebsocketMessage,
    WebsocketSession,
}

impl FrameworkAttribute {
    pub const ALL: [Self; 32] = [
        Self::AuthenticatedUser,
        Self::BuildForSession,
        Self::Column,
        Self::ConsoleArgument,
        Self::ConsoleCommand,
        Self::Constructor,
        Self::EnvironmentVariable,
        Self::ForeignKey,
        Self::FormRequest,
        Self::HandlesMiddlewareAttribute,
        Self::Index,
        Self::InferFromRequest,
        Self::InfersAuthenticatedUser,
        Self::JwksSecretStore,
        Self::Middleware,
        Self::Model,
        Self::PrimaryKey,
        Self::Process,
        Self::ProvidesJwksEndpoint,
        Self::ProvidesRouteParameter,
        Self::RendersView,
        Self::RespondsToHttp,
        Self::RouteParameter,
        Self::RouteParameterValue,
        Self::ScheduledWithTickTimer,
        Self::Service,
        Self::Singleton,
        Self::SpiffeHttpClient,
        Self::SpiffeWebsocketClient,
        Self::Unique,
        Self::WebsocketMessage,
        Self::WebsocketSession,
    ];

    #[must_use]
    pub fn from_canonical_path(path: &CanonicalPath) -> Option<Self> {
        let name = match path.segments() {
            [name] => name.as_str(),
            [crate_name, name] if crate_name == "margaret_macros" => name.as_str(),
            [crate_name, framework, macros, name]
                if crate_name == "margaret" && framework == "framework" && macros == "macros" =>
            {
                name.as_str()
            }
            _ => return None,
        };

        match name {
            "authenticated_user" => Some(Self::AuthenticatedUser),
            "build_for_session" => Some(Self::BuildForSession),
            "column" => Some(Self::Column),
            "console_argument" => Some(Self::ConsoleArgument),
            "console_command" => Some(Self::ConsoleCommand),
            "constructor" => Some(Self::Constructor),
            "environment_variable" => Some(Self::EnvironmentVariable),
            "foreign_key" => Some(Self::ForeignKey),
            "form_request" => Some(Self::FormRequest),
            "handles_middleware_attribute" => Some(Self::HandlesMiddlewareAttribute),
            "index" => Some(Self::Index),
            "infer_from_request" => Some(Self::InferFromRequest),
            "infers_authenticated_user" => Some(Self::InfersAuthenticatedUser),
            "jwks_secret_store" => Some(Self::JwksSecretStore),
            "middleware" => Some(Self::Middleware),
            "model" => Some(Self::Model),
            "primary_key" => Some(Self::PrimaryKey),
            "process" => Some(Self::Process),
            "provides_jwks_endpoint" => Some(Self::ProvidesJwksEndpoint),
            "provides_route_parameter" => Some(Self::ProvidesRouteParameter),
            "renders_view" => Some(Self::RendersView),
            "responds_to_http" => Some(Self::RespondsToHttp),
            "route_parameter" => Some(Self::RouteParameter),
            "route_parameter_value" => Some(Self::RouteParameterValue),
            "scheduled_with_tick_timer" => Some(Self::ScheduledWithTickTimer),
            "service" => Some(Self::Service),
            "singleton" => Some(Self::Singleton),
            "spiffe_http_client" => Some(Self::SpiffeHttpClient),
            "spiffe_websocket_client" => Some(Self::SpiffeWebsocketClient),
            "unique" => Some(Self::Unique),
            "websocket_message" => Some(Self::WebsocketMessage),
            "websocket_session" => Some(Self::WebsocketSession),
            _ => None,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AuthenticatedUser => "authenticated_user",
            Self::BuildForSession => "build_for_session",
            Self::Column => "column",
            Self::ConsoleArgument => "console_argument",
            Self::ConsoleCommand => "console_command",
            Self::Constructor => "constructor",
            Self::EnvironmentVariable => "environment_variable",
            Self::ForeignKey => "foreign_key",
            Self::FormRequest => "form_request",
            Self::HandlesMiddlewareAttribute => "handles_middleware_attribute",
            Self::Index => "index",
            Self::InferFromRequest => "infer_from_request",
            Self::InfersAuthenticatedUser => "infers_authenticated_user",
            Self::JwksSecretStore => "jwks_secret_store",
            Self::Middleware => "middleware",
            Self::Model => "model",
            Self::PrimaryKey => "primary_key",
            Self::Process => "process",
            Self::ProvidesJwksEndpoint => "provides_jwks_endpoint",
            Self::ProvidesRouteParameter => "provides_route_parameter",
            Self::RendersView => "renders_view",
            Self::RespondsToHttp => "responds_to_http",
            Self::RouteParameter => "route_parameter",
            Self::RouteParameterValue => "route_parameter_value",
            Self::ScheduledWithTickTimer => "scheduled_with_tick_timer",
            Self::Service => "service",
            Self::Singleton => "singleton",
            Self::SpiffeHttpClient => "spiffe_http_client",
            Self::SpiffeWebsocketClient => "spiffe_websocket_client",
            Self::Unique => "unique",
            Self::WebsocketMessage => "websocket_message",
            Self::WebsocketSession => "websocket_session",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canonical_path::CanonicalPath;

    use super::FrameworkAttribute;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(
            segments
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
        )
    }

    #[test]
    fn recognizes_every_supported_attribute_through_each_canonical_macro_path() {
        for attribute in FrameworkAttribute::ALL {
            let name = attribute.name();

            assert_eq!(
                FrameworkAttribute::from_canonical_path(&path(&[name])),
                Some(attribute)
            );
            assert_eq!(
                FrameworkAttribute::from_canonical_path(&path(&["margaret_macros", name])),
                Some(attribute)
            );
            assert_eq!(
                FrameworkAttribute::from_canonical_path(&path(&[
                    "margaret",
                    "framework",
                    "macros",
                    name,
                ])),
                Some(attribute)
            );
        }
    }

    #[test]
    fn rejects_paths_that_do_not_identify_a_supported_framework_attribute() {
        assert_eq!(
            FrameworkAttribute::from_canonical_path(&path(&["other", "singleton"])),
            None
        );
        assert_eq!(
            FrameworkAttribute::from_canonical_path(&path(&["unknown"])),
            None
        );
        assert_eq!(FrameworkAttribute::from_canonical_path(&path(&[])), None);
    }
}
