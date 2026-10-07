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
    ExchangesSubjectTokens,
}

impl SingletonDeclaration {
    pub(crate) const ALL: [Self; 1] = [Self::ExchangesSubjectTokens];

    pub(crate) fn attribute(self) -> FrameworkAttribute {
        match self {
            Self::ExchangesSubjectTokens => FrameworkAttribute::ExchangesSubjectTokens,
        }
    }

    pub(crate) fn required_trait(self) -> CanonicalPath {
        match self {
            Self::ExchangesSubjectTokens => framework_path(&[
                "subject_token_exchange",
                "exchanges_subject_tokens",
                "ExchangesSubjectTokens",
            ]),
        }
    }
}
