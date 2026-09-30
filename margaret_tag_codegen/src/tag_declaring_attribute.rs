use margaret_attributes::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy)]
pub(crate) enum TagDeclaringAttribute {
    HandlesMiddlewareAttribute,
    OAuthClient,
    ProvidesJwksEndpoint,
    TrustsOidcIssuer,
}

impl TagDeclaringAttribute {
    pub(crate) const ALL: [Self; 4] = [
        Self::ProvidesJwksEndpoint,
        Self::HandlesMiddlewareAttribute,
        Self::TrustsOidcIssuer,
        Self::OAuthClient,
    ];

    pub(crate) fn framework_attribute(self) -> FrameworkAttribute {
        match self {
            Self::HandlesMiddlewareAttribute => FrameworkAttribute::HandlesMiddlewareAttribute,
            Self::OAuthClient => FrameworkAttribute::OAuthClient,
            Self::ProvidesJwksEndpoint => FrameworkAttribute::ProvidesJwksEndpoint,
            Self::TrustsOidcIssuer => FrameworkAttribute::TrustsOidcIssuer,
        }
    }
}
