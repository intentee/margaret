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
    ExchangesTokensFrom,
    RemembersClientAssertions,
    StoresAuthorizationGrants,
    StoresSigningKeys,
}

impl SingletonDeclaration {
    pub(crate) const ALL: [Self; 4] = [
        Self::ExchangesTokensFrom,
        Self::RemembersClientAssertions,
        Self::StoresAuthorizationGrants,
        Self::StoresSigningKeys,
    ];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::ExchangesTokensFrom => FrameworkAttribute::ExchangesTokensFrom,
            Self::RemembersClientAssertions => FrameworkAttribute::RemembersClientAssertions,
            Self::StoresAuthorizationGrants => FrameworkAttribute::StoresAuthorizationGrants,
            Self::StoresSigningKeys => FrameworkAttribute::StoresSigningKeys,
        }
    }

    pub(crate) fn required_trait(self) -> CanonicalPath {
        match self {
            Self::ExchangesTokensFrom => framework_path(&[
                "subject_token_exchange",
                "exchanges_subject_tokens",
                "ExchangesSubjectTokens",
            ]),
            Self::RemembersClientAssertions => framework_path(&[
                "accepted_clients",
                "remembers_client_assertions",
                "RemembersClientAssertions",
            ]),
            Self::StoresAuthorizationGrants => framework_path(&[
                "authorization_grants",
                "stores_authorization_grants",
                "StoresAuthorizationGrants",
            ]),
            Self::StoresSigningKeys => {
                framework_path(&["jwks_roller", "stores_signing_keys", "StoresSigningKeys"])
            }
        }
    }
}
