use margaret_attributes::framework_attribute::FrameworkAttribute;

pub(crate) const DECLARATION_ATTRIBUTES: [FrameworkAttribute; 12] = [
    FrameworkAttribute::AdmitsOAuthClient,
    FrameworkAttribute::ConsumesSessions,
    FrameworkAttribute::IssuesResourceTokens,
    FrameworkAttribute::IssuesSessions,
    FrameworkAttribute::IssuesTokens,
    FrameworkAttribute::OAuthClient,
    FrameworkAttribute::OAuthScope,
    FrameworkAttribute::PostgresDatabase,
    FrameworkAttribute::ServesOidcEndpoint,
    FrameworkAttribute::ServesSessionEndpoint,
    FrameworkAttribute::ServesSignIn,
    FrameworkAttribute::VerifiesTokensFromIssuer,
];
