use margaret_attributes::framework_attribute::FrameworkAttribute;

#[derive(Clone, Copy)]
pub(crate) enum TrustAttribute {
    ProvidesJwksEndpoint,
    TrustsOidcIssuer,
}

impl TrustAttribute {
    pub(crate) const ALL: [Self; 2] = [Self::ProvidesJwksEndpoint, Self::TrustsOidcIssuer];

    pub(crate) fn framework_attribute(self) -> FrameworkAttribute {
        match self {
            Self::ProvidesJwksEndpoint => FrameworkAttribute::ProvidesJwksEndpoint,
            Self::TrustsOidcIssuer => FrameworkAttribute::TrustsOidcIssuer,
        }
    }
}
