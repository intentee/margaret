use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::issuer_identifier::IssuerIdentifier;
use crate::registered_claims_error::RegisteredClaimsError;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Audience {
    value: String,
}

impl Audience {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl Display for Audience {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.value)
    }
}

impl FromStr for Audience {
    type Err = RegisteredClaimsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(RegisteredClaimsError::AudienceEmpty);
        }

        Ok(Self {
            value: value.to_string(),
        })
    }
}

impl From<&IssuerIdentifier> for Audience {
    fn from(issuer: &IssuerIdentifier) -> Self {
        Self {
            value: issuer.as_str().to_string(),
        }
    }
}
