use margaret_attributes::framework_attribute::FrameworkAttribute;

pub(crate) const DECLARATION_ATTRIBUTES: [FrameworkAttribute; 5] = [
    FrameworkAttribute::ActsAsOAuthClient,
    FrameworkAttribute::AdmitsOAuthClient,
    FrameworkAttribute::IssuesResourceTokens,
    FrameworkAttribute::IssuesTokens,
    FrameworkAttribute::VerifiesTokensFromIssuer,
];
