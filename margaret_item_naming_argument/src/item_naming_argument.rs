use crate::named_item::NamedItem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemNamingArgument {
    ClientAuthentication,
    ColumnDefault,
    Consent,
    Cookies,
    FormRequestSource,
    IdTokenSigning,
    Keys,
    LandingRoute,
    OnDelete,
    RedirectRoutes,
    RelationModel,
    RouteMethod,
    Scopes,
    Signing,
    TickBehavior,
    TickInterval,
    UserModel,
    View,
    WebSocketResponse,
}

impl ItemNamingArgument {
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::ClientAuthentication => "authentication",
            Self::ColumnDefault => "default",
            Self::Consent => "consent",
            Self::Cookies => "cookies",
            Self::FormRequestSource => "from",
            Self::IdTokenSigning => "id_token_signing",
            Self::Keys => "keys",
            Self::LandingRoute => "landing_route",
            Self::OnDelete => "on_delete",
            Self::RedirectRoutes => "redirect_routes",
            Self::RelationModel => "model",
            Self::RouteMethod => "method",
            Self::Scopes => "scopes",
            Self::Signing => "signing",
            Self::TickBehavior => "behavior",
            Self::TickInterval => "interval",
            Self::UserModel => "user_model",
            Self::View => "view",
            Self::WebSocketResponse => "response",
        }
    }

    #[must_use]
    pub fn named_item(self) -> NamedItem {
        match self {
            Self::LandingRoute
            | Self::RedirectRoutes
            | Self::RelationModel
            | Self::Scopes
            | Self::UserModel
            | Self::View => NamedItem::Type,
            Self::ClientAuthentication
            | Self::ColumnDefault
            | Self::Consent
            | Self::Cookies
            | Self::FormRequestSource
            | Self::IdTokenSigning
            | Self::Keys
            | Self::OnDelete
            | Self::RouteMethod
            | Self::Signing
            | Self::TickBehavior
            | Self::TickInterval
            | Self::WebSocketResponse => NamedItem::Value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ItemNamingArgument;
    use crate::named_item::NamedItem;

    #[test]
    fn an_argument_naming_a_model_a_route_a_scope_or_a_view_names_a_type() {
        assert_eq!(
            [
                ItemNamingArgument::LandingRoute,
                ItemNamingArgument::RedirectRoutes,
                ItemNamingArgument::RelationModel,
                ItemNamingArgument::Scopes,
                ItemNamingArgument::UserModel,
                ItemNamingArgument::View,
            ]
            .map(ItemNamingArgument::named_item),
            [NamedItem::Type; 6]
        );
    }

    #[test]
    fn an_argument_naming_a_framework_value_names_a_value() {
        assert_eq!(
            [
                ItemNamingArgument::ClientAuthentication,
                ItemNamingArgument::ColumnDefault,
                ItemNamingArgument::Consent,
                ItemNamingArgument::Cookies,
                ItemNamingArgument::FormRequestSource,
                ItemNamingArgument::IdTokenSigning,
                ItemNamingArgument::Keys,
                ItemNamingArgument::OnDelete,
                ItemNamingArgument::RouteMethod,
                ItemNamingArgument::Signing,
                ItemNamingArgument::TickBehavior,
                ItemNamingArgument::TickInterval,
                ItemNamingArgument::WebSocketResponse,
            ]
            .map(ItemNamingArgument::named_item),
            [NamedItem::Value; 13]
        );
    }
}
