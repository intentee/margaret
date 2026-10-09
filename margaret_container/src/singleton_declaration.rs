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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SingletonDeclaration {
    AdmitsSignIn,
    ExchangesTokensFrom,
    ProvidesUserinfoClaims,
}

impl SingletonDeclaration {
    pub(crate) const ALL: [Self; 3] = [
        Self::AdmitsSignIn,
        Self::ExchangesTokensFrom,
        Self::ProvidesUserinfoClaims,
    ];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::AdmitsSignIn => FrameworkAttribute::AdmitsSignIn,
            Self::ExchangesTokensFrom => FrameworkAttribute::ExchangesTokensFrom,
            Self::ProvidesUserinfoClaims => FrameworkAttribute::ProvidesUserinfoClaims,
        }
    }

    pub(crate) fn required_trait(self) -> CanonicalPath {
        match self {
            Self::AdmitsSignIn => {
                framework_path(&["oidc_sign_in", "admits_sign_in", "AdmitsSignIn"])
            }
            Self::ExchangesTokensFrom => framework_path(&[
                "subject_token_exchange",
                "exchanges_subject_tokens",
                "ExchangesSubjectTokens",
            ]),
            Self::ProvidesUserinfoClaims => framework_path(&[
                "oidc_provider",
                "provides_userinfo_claims",
                "ProvidesUserinfoClaims",
            ]),
        }
    }
}
