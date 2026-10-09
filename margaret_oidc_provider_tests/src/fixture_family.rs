use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_registered_claims::numeric_date::NumericDate;

pub(crate) enum FixtureFamily {
    Open(RefreshFamily),
    Revoked { expires_at: NumericDate },
}

impl FixtureFamily {
    pub(crate) fn expires_at(&self) -> NumericDate {
        match self {
            Self::Open(record) => record.expires_at,
            Self::Revoked { expires_at } => *expires_at,
        }
    }
}
