use margaret_oauth_vocabulary::code_verifier_rejection::CodeVerifierRejection;

pub(crate) enum CodeRefusal {
    MalformedVerifier(CodeVerifierRejection),
    Mismatch,
}

impl CodeRefusal {
    pub(crate) fn description(&self) -> &'static str {
        match self {
            Self::MalformedVerifier(CodeVerifierRejection::ReservedCharacter { .. }) => {
                "the code verifier contains a character outside the unreserved set"
            }
            Self::MalformedVerifier(CodeVerifierRejection::TooLong { .. }) => {
                "the code verifier is longer than 128 characters"
            }
            Self::MalformedVerifier(CodeVerifierRejection::TooShort { .. }) => {
                "the code verifier is shorter than 43 characters"
            }
            Self::Mismatch => {
                "the authorization code was granted to another client, callback or verifier"
            }
        }
    }
}
