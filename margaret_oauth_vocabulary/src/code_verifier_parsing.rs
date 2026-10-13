use crate::code_verifier::CodeVerifier;
use crate::code_verifier_rejection::CodeVerifierRejection;

pub enum CodeVerifierParsing {
    Accepted(CodeVerifier),
    Rejected(CodeVerifierRejection),
}
