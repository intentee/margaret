use crate::named_item::NamedItem;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemNamingArgument {
    ClientAuthentication,
    ColumnDefault,
    Consent,
    FormRequestSource,
    IdTokenSigning,
    Keys,
    OnDelete,
    RedirectRoute,
    RedirectRoutes,
    RelationModel,
    RouteMethod,
    Signing,
    TickBehavior,
    TickInterval,
    UserModel,
    WebSocketResponse,
}

impl ItemNamingArgument {
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::ClientAuthentication => "authentication",
            Self::ColumnDefault => "default",
            Self::Consent => "consent",
            Self::FormRequestSource => "from",
            Self::IdTokenSigning => "id_token_signing",
            Self::Keys => "keys",
            Self::OnDelete => "on_delete",
            Self::RedirectRoute => "redirect_route",
            Self::RedirectRoutes => "redirect_routes",
            Self::RelationModel => "model",
            Self::RouteMethod => "method",
            Self::Signing => "signing",
            Self::TickBehavior => "behavior",
            Self::TickInterval => "interval",
            Self::UserModel => "user_model",
            Self::WebSocketResponse => "response",
        }
    }

    #[must_use]
    pub fn named_item(self) -> NamedItem {
        match self {
            Self::RedirectRoute | Self::RedirectRoutes | Self::RelationModel | Self::UserModel => {
                NamedItem::Type
            }
            Self::ClientAuthentication
            | Self::ColumnDefault
            | Self::Consent
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
    fn an_argument_naming_a_model_or_a_route_names_a_type() {
        assert_eq!(
            [
                ItemNamingArgument::RedirectRoute,
                ItemNamingArgument::RedirectRoutes,
                ItemNamingArgument::RelationModel,
                ItemNamingArgument::UserModel,
            ]
            .map(ItemNamingArgument::named_item),
            [NamedItem::Type; 4]
        );
    }

    #[test]
    fn an_argument_naming_a_framework_value_names_a_value() {
        assert_eq!(
            [
                ItemNamingArgument::ClientAuthentication,
                ItemNamingArgument::ColumnDefault,
                ItemNamingArgument::Consent,
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
            [NamedItem::Value; 12]
        );
    }
}
