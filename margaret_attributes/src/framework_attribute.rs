use crate::canonical_path::CanonicalPath;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FrameworkAttribute {
    ActsAsOAuthClient,
    AdmitsOAuthClient,
    AuthenticatedUser,
    BearerToken,
    BuildForSession,
    Column,
    ConsoleArgument,
    ConsoleCommand,
    Constructor,
    EnvironmentVariable,
    ExchangesTokensFrom,
    ForeignKey,
    FormRequest,
    HandlesMiddlewareAttribute,
    Index,
    InferFromRequest,
    InfersAuthenticatedUser,
    IssuesResourceTokens,
    IssuesTokens,
    Middleware,
    Model,
    PrimaryKey,
    Process,
    ProvidesRouteParameter,
    RemembersClientAssertions,
    RendersView,
    RespondsToHttp,
    RouteParameter,
    RouteParameterValue,
    ScheduledWithTickTimer,
    Service,
    Singleton,
    SpiffeHttpClient,
    StoresAuthorizationGrants,
    StoresSigningKeys,
    Unique,
    VerifiesTokensFromIssuer,
    WebsocketMessage,
    WebsocketSession,
}

impl FrameworkAttribute {
    pub const ALL: [Self; 39] = [
        Self::ActsAsOAuthClient,
        Self::AdmitsOAuthClient,
        Self::AuthenticatedUser,
        Self::BearerToken,
        Self::BuildForSession,
        Self::Column,
        Self::ConsoleArgument,
        Self::ConsoleCommand,
        Self::Constructor,
        Self::EnvironmentVariable,
        Self::ExchangesTokensFrom,
        Self::ForeignKey,
        Self::FormRequest,
        Self::HandlesMiddlewareAttribute,
        Self::Index,
        Self::InferFromRequest,
        Self::InfersAuthenticatedUser,
        Self::IssuesResourceTokens,
        Self::IssuesTokens,
        Self::Middleware,
        Self::Model,
        Self::PrimaryKey,
        Self::Process,
        Self::ProvidesRouteParameter,
        Self::RemembersClientAssertions,
        Self::RendersView,
        Self::RespondsToHttp,
        Self::RouteParameter,
        Self::RouteParameterValue,
        Self::ScheduledWithTickTimer,
        Self::Service,
        Self::Singleton,
        Self::SpiffeHttpClient,
        Self::StoresAuthorizationGrants,
        Self::StoresSigningKeys,
        Self::Unique,
        Self::VerifiesTokensFromIssuer,
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

        Self::ALL
            .into_iter()
            .find(|attribute| attribute.name() == name)
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ActsAsOAuthClient => "acts_as_oauth_client",
            Self::AdmitsOAuthClient => "admits_oauth_client",
            Self::AuthenticatedUser => "authenticated_user",
            Self::BearerToken => "bearer_token",
            Self::BuildForSession => "build_for_session",
            Self::Column => "column",
            Self::ConsoleArgument => "console_argument",
            Self::ConsoleCommand => "console_command",
            Self::Constructor => "constructor",
            Self::EnvironmentVariable => "environment_variable",
            Self::ExchangesTokensFrom => "exchanges_tokens_from",
            Self::ForeignKey => "foreign_key",
            Self::FormRequest => "form_request",
            Self::HandlesMiddlewareAttribute => "handles_middleware_attribute",
            Self::Index => "index",
            Self::InferFromRequest => "infer_from_request",
            Self::InfersAuthenticatedUser => "infers_authenticated_user",
            Self::IssuesResourceTokens => "issues_resource_tokens",
            Self::IssuesTokens => "issues_tokens",
            Self::Middleware => "middleware",
            Self::Model => "model",
            Self::PrimaryKey => "primary_key",
            Self::Process => "process",
            Self::ProvidesRouteParameter => "provides_route_parameter",
            Self::RemembersClientAssertions => "remembers_client_assertions",
            Self::RendersView => "renders_view",
            Self::RespondsToHttp => "responds_to_http",
            Self::RouteParameter => "route_parameter",
            Self::RouteParameterValue => "route_parameter_value",
            Self::ScheduledWithTickTimer => "scheduled_with_tick_timer",
            Self::Service => "service",
            Self::Singleton => "singleton",
            Self::SpiffeHttpClient => "spiffe_http_client",
            Self::StoresAuthorizationGrants => "stores_authorization_grants",
            Self::StoresSigningKeys => "stores_signing_keys",
            Self::Unique => "unique",
            Self::VerifiesTokensFromIssuer => "verifies_tokens_from_issuer",
            Self::WebsocketMessage => "websocket_message",
            Self::WebsocketSession => "websocket_session",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FrameworkAttribute;
    use crate::canonical_path::CanonicalPath;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
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
