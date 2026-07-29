use crate::token_malformation::TokenMalformation;

pub enum TokenVerification<TClaims> {
    Malformed(TokenMalformation),
    SignatureMismatch,
    Verified(TClaims),
}

impl<TClaims> TokenVerification<TClaims> {
    pub fn verified(self) -> Option<TClaims> {
        match self {
            Self::Malformed(_) | Self::SignatureMismatch => None,
            Self::Verified(claims) => Some(claims),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TokenVerification;
    use crate::token_malformation::TokenMalformation;

    #[test]
    fn reports_claims_only_for_a_verified_token() {
        assert_eq!(TokenVerification::Verified(7).verified(), Some(7));
        assert_eq!(TokenVerification::<i32>::SignatureMismatch.verified(), None);
        assert_eq!(
            TokenVerification::<i32>::Malformed(TokenMalformation::NotCompactJws).verified(),
            None
        );
    }
}
