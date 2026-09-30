use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthenticationAge {
    AtMost(TimeDelta),
    Unbounded,
}

impl AuthenticationAge {
    pub(crate) fn admits(self, authenticated_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        match self {
            Self::AtMost(max_age) => now - authenticated_at <= max_age,
            Self::Unbounded => true,
        }
    }
}
