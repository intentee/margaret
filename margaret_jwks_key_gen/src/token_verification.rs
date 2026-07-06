use crate::jwks_key_error::JwksKeyError;

pub enum TokenVerification<TClaims> {
    SignatureMismatch,
    Verified(TClaims),
}

impl<TClaims> TokenVerification<TClaims> {
    pub fn must(self) -> Result<TClaims, JwksKeyError> {
        match self {
            Self::SignatureMismatch => Err(JwksKeyError::SignatureMismatch),
            Self::Verified(claims) => Ok(claims),
        }
    }
}
