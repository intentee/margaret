use crate::token_malformation::TokenMalformation;

pub(crate) enum SignatureCheck {
    Malformed(TokenMalformation),
    Matches,
    Mismatch,
}

impl SignatureCheck {
    pub(crate) fn from_verification(matches: bool) -> Self {
        if matches {
            Self::Matches
        } else {
            Self::Mismatch
        }
    }
}
