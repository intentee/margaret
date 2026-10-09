use crate::code_challenge_method_parsing::CodeChallengeMethodParsing;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeChallengeMethod {
    S256,
}

impl CodeChallengeMethod {
    #[must_use]
    pub fn parse(value: &str) -> CodeChallengeMethodParsing {
        if value == Self::S256.wire_name() {
            CodeChallengeMethodParsing::Accepted(Self::S256)
        } else {
            CodeChallengeMethodParsing::Unsupported
        }
    }

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::S256 => "S256",
        }
    }
}
