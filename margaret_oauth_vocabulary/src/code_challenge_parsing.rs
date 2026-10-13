use crate::code_challenge::CodeChallenge;
use crate::code_challenge_rejection::CodeChallengeRejection;

pub enum CodeChallengeParsing {
    Accepted(CodeChallenge),
    Rejected(CodeChallengeRejection),
}
