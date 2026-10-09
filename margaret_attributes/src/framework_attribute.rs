use syn::Path;

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
    PostgresDatabase,
    PrimaryKey,
    Process,
    ProvidesRouteParameter,
    RendersView,
    RespondsToHttp,
    RouteParameter,
    RouteParameterValue,
    ScheduledWithTickTimer,
    Service,
    Singleton,
    SpiffeHttpClient,
    Unique,
    VerifiesTokensFromIssuer,
    WebsocketMessage,
    WebsocketSession,
}

impl FrameworkAttribute {
    pub const ALL: [Self; 37] = [
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
        Self::PostgresDatabase,
        Self::PrimaryKey,
        Self::Process,
        Self::ProvidesRouteParameter,
        Self::RendersView,
        Self::RespondsToHttp,
        Self::RouteParameter,
        Self::RouteParameterValue,
        Self::ScheduledWithTickTimer,
        Self::Service,
        Self::Singleton,
        Self::SpiffeHttpClient,
        Self::Unique,
        Self::VerifiesTokensFromIssuer,
        Self::WebsocketMessage,
        Self::WebsocketSession,
    ];

    #[must_use]
    pub fn recognize(written: &Path, canonical: &CanonicalPath) -> Option<Self> {
        written
            .get_ident()
            .and_then(|identifier| {
                Self::ALL
                    .into_iter()
                    .find(|attribute| attribute.is_marker() && identifier == attribute.name())
            })
            .or_else(|| Self::macro_at(canonical))
    }

    #[must_use]
    pub const fn is_marker(self) -> bool {
        matches!(
            self,
            Self::AuthenticatedUser
                | Self::BearerToken
                | Self::Column
                | Self::ConsoleArgument
                | Self::EnvironmentVariable
                | Self::ForeignKey
                | Self::FormRequest
                | Self::Index
                | Self::PrimaryKey
                | Self::RouteParameter
                | Self::SpiffeHttpClient
                | Self::Unique
        )
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
            Self::PostgresDatabase => "postgres_database",
            Self::PrimaryKey => "primary_key",
            Self::Process => "process",
            Self::ProvidesRouteParameter => "provides_route_parameter",
            Self::RendersView => "renders_view",
            Self::RespondsToHttp => "responds_to_http",
            Self::RouteParameter => "route_parameter",
            Self::RouteParameterValue => "route_parameter_value",
            Self::ScheduledWithTickTimer => "scheduled_with_tick_timer",
            Self::Service => "service",
            Self::Singleton => "singleton",
            Self::SpiffeHttpClient => "spiffe_http_client",
            Self::Unique => "unique",
            Self::VerifiesTokensFromIssuer => "verifies_tokens_from_issuer",
            Self::WebsocketMessage => "websocket_message",
            Self::WebsocketSession => "websocket_session",
        }
    }

    pub(crate) const fn repeats_on_items(self) -> bool {
        matches!(
            self,
            Self::ForeignKey | Self::Index | Self::Middleware | Self::Unique
        )
    }

    fn macro_at(canonical: &CanonicalPath) -> Option<Self> {
        let name = match canonical.segments() {
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
            .find(|attribute| !attribute.is_marker() && attribute.name() == name)
    }
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_str;

    use super::FrameworkAttribute;
    use crate::canonical_path::CanonicalPath;

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(ToString::to_string).collect())
    }

    fn written(source: &str) -> Path {
        parse_str(source).expect("the attribute path parses")
    }

    fn shadowing_item() -> CanonicalPath {
        path(&["crate", "models", "shadowing_item"])
    }

    #[test]
    fn recognizes_every_macro_through_each_canonical_macro_path() {
        for attribute in FrameworkAttribute::ALL
            .into_iter()
            .filter(|attribute| !attribute.is_marker())
        {
            let name = attribute.name();

            for canonical in [
                path(&[name]),
                path(&["margaret_macros", name]),
                path(&["margaret", "framework", "macros", name]),
            ] {
                assert_eq!(
                    FrameworkAttribute::recognize(&written(name), &canonical),
                    Some(attribute)
                );
            }
        }
    }

    #[test]
    fn recognizes_every_marker_by_its_written_name_whatever_it_resolves_to() {
        for attribute in FrameworkAttribute::ALL
            .into_iter()
            .filter(|attribute| attribute.is_marker())
        {
            assert_eq!(
                FrameworkAttribute::recognize(&written(attribute.name()), &shadowing_item()),
                Some(attribute)
            );
        }
    }

    #[test]
    fn ignores_a_marker_spelled_as_a_macro_path() {
        assert_eq!(
            FrameworkAttribute::recognize(
                &written("margaret::framework::macros::column"),
                &path(&["margaret", "framework", "macros", "column"]),
            ),
            None
        );
    }

    #[test]
    fn ignores_a_macro_name_that_resolves_to_another_item() {
        assert_eq!(
            FrameworkAttribute::recognize(&written("singleton"), &shadowing_item()),
            None
        );
    }

    #[test]
    fn ignores_a_path_that_names_no_framework_attribute() {
        assert_eq!(
            FrameworkAttribute::recognize(
                &written("other::singleton"),
                &path(&["other", "singleton"])
            ),
            None
        );
    }
}
