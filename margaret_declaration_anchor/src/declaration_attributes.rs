use margaret_attributes::framework_attribute::FrameworkAttribute;

pub(crate) const DECLARATION_ATTRIBUTES: [FrameworkAttribute; 5] = [
    FrameworkAttribute::AcceptsOAuthClient,
    FrameworkAttribute::IssuesTokens,
    FrameworkAttribute::OAuthClient,
    FrameworkAttribute::ProvidesJwksEndpoint,
    FrameworkAttribute::TrustsOidcIssuer,
];
