use crate::code_challenge_method::CodeChallengeMethod;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeChallengeMethodParsing {
    Accepted(CodeChallengeMethod),
    Unsupported,
}
