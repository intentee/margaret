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
    IssuesTokens,
    ProvidesJwksEndpoint,
    TrustsOidcIssuer,
}

impl SingletonDeclaration {
    pub(crate) const ALL: [Self; 3] = [
        Self::IssuesTokens,
        Self::ProvidesJwksEndpoint,
        Self::TrustsOidcIssuer,
    ];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::IssuesTokens => FrameworkAttribute::IssuesTokens,
            Self::ProvidesJwksEndpoint => FrameworkAttribute::ProvidesJwksEndpoint,
            Self::TrustsOidcIssuer => FrameworkAttribute::TrustsOidcIssuer,
        }
    }

    pub(crate) fn required_traits(self) -> Vec<CanonicalPath> {
        match self {
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
