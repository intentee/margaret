use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;

fn framework_path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(
        ["margaret", "framework"]
            .iter()
            .chain(segments)
            .map(|segment| (*segment).to_string())
            .collect(),
    )
}

#[derive(Clone, Copy)]
pub(crate) enum SingletonDeclaration {
    AcceptsOAuthClient,
    ExchangesSubjectTokens,
    IssuesTokens,
    ProvidesJwksEndpoint,
    TrustsOidcIssuer,
}

impl SingletonDeclaration {
    pub(crate) const ALL: [Self; 5] = [
        Self::AcceptsOAuthClient,
        Self::ExchangesSubjectTokens,
        Self::IssuesTokens,
        Self::ProvidesJwksEndpoint,
        Self::TrustsOidcIssuer,
    ];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::AcceptsOAuthClient => FrameworkAttribute::AcceptsOAuthClient,
            Self::ExchangesSubjectTokens => FrameworkAttribute::ExchangesSubjectTokens,
            Self::IssuesTokens => FrameworkAttribute::IssuesTokens,
            Self::ProvidesJwksEndpoint => FrameworkAttribute::ProvidesJwksEndpoint,
            Self::TrustsOidcIssuer => FrameworkAttribute::TrustsOidcIssuer,
        }
    }

    pub(crate) fn required_traits(self) -> Vec<CanonicalPath> {
        match self {
            Self::AcceptsOAuthClient => vec![framework_path(&[
                "accepted_clients",
                "declares_accepted_client",
                "DeclaresAcceptedClient",
            ])],
            Self::ExchangesSubjectTokens => vec![framework_path(&[
                "subject_token_exchange",
                "exchanges_subject_tokens",
                "ExchangesSubjectTokens",
            ])],
            Self::IssuesTokens => vec![framework_path(&[
                "token_issuance",
                "declares_token_issuance",
                "DeclaresTokenIssuance",
            ])],
            Self::ProvidesJwksEndpoint => vec![
                framework_path(&["jwks_endpoint", "provides_endpoint", "ProvidesEndpoint"]),
                framework_path(&["token_trust", "declares_token_trust", "DeclaresTokenTrust"]),
            ],
            Self::TrustsOidcIssuer => vec![framework_path(&[
                "token_trust",
                "declares_token_trust",
                "DeclaresTokenTrust",
            ])],
        }
    }
}
