use crate::prompt_value_parsing::PromptValueParsing;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PromptValue {
    Consent,
    Login,
    None,
}

impl PromptValue {
    pub const SUPPORTED: [Self; 3] = [Self::Consent, Self::Login, Self::None];

    #[must_use]
    pub fn parse(value: &str) -> PromptValueParsing {
        Self::SUPPORTED
            .into_iter()
            .find(|supported| supported.wire_name() == value)
            .map_or(
                PromptValueParsing::Unsupported,
                PromptValueParsing::Accepted,
            )
    }

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Consent => "consent",
            Self::Login => "login",
            Self::None => "none",
        }
    }
}
