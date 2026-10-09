use uuid::Uuid;

use margaret_authorization_grants::issued_code::IssuedCode;
use margaret_registered_claims::numeric_date::NumericDate;

pub(crate) enum FixtureCode {
    Issued(Box<IssuedCode>),
    Redeemed {
        expires_at: NumericDate,
        family: Uuid,
    },
}

impl FixtureCode {
    pub(crate) fn expires_at(&self) -> NumericDate {
        match self {
            Self::Issued(issued) => issued.expires_at,
            Self::Redeemed { expires_at, .. } => *expires_at,
        }
    }
}
