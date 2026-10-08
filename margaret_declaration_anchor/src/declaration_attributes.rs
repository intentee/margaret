use margaret_attributes::framework_attribute::FrameworkAttribute;

pub(crate) const DECLARATION_ATTRIBUTES: [FrameworkAttribute; 6] = [
    FrameworkAttribute::ActsAsOAuthClient,
    FrameworkAttribute::AdmitsOAuthClient,
    FrameworkAttribute::IssuesResourceTokens,
    FrameworkAttribute::IssuesTokens,
    FrameworkAttribute::PostgresDatabase,
    FrameworkAttribute::VerifiesTokensFromIssuer,
];
