use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AuthenticationAge {
    AtMost(TimeDelta),
    NewLogin,
    Unbounded,
}

impl AuthenticationAge {
    pub(crate) fn admits(self, authenticated_at: DateTime<Utc>, now: DateTime<Utc>) -> bool {
        match self {
            Self::AtMost(max_age) => now - authenticated_at <= max_age,
            Self::NewLogin => false,
            Self::Unbounded => true,
        }
    }

    pub(crate) fn after_login(self) -> Option<String> {
        match self {
            Self::AtMost(max_age) => Some(max_age.num_seconds().to_string()),
            Self::NewLogin | Self::Unbounded => None,
        }
    }
}
