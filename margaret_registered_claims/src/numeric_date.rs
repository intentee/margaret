use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use crate::registered_claims_error::RegisteredClaimsError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NumericDate {
    seconds_since_epoch: i64,
}

impl NumericDate {
    fn from_duration_since_epoch(
        duration: Duration,
    ) -> std::result::Result<Self, RegisteredClaimsError> {
        i64::try_from(duration.as_secs())
            .map(Self::new)
            .map_err(|source| RegisteredClaimsError::ClockBeyondNumericDate { source })
    }

    /// # Errors
    ///
    /// Returns `RegisteredClaimsError::ClockBeforeUnixEpoch` when the time precedes the unix epoch, and
    /// `RegisteredClaimsError::ClockBeyondNumericDate` when its seconds exceed the numeric date range.
    pub fn from_system_time(time: SystemTime) -> std::result::Result<Self, RegisteredClaimsError> {
        time.duration_since(UNIX_EPOCH)
            .map_err(|source| RegisteredClaimsError::ClockBeforeUnixEpoch { source })
            .and_then(Self::from_duration_since_epoch)
    }

    #[must_use]
    pub fn new(seconds_since_epoch: i64) -> Self {
        Self {
            seconds_since_epoch,
        }
    }

    #[must_use]
    pub fn seconds_since_epoch(self) -> i64 {
        self.seconds_since_epoch
    }
}

impl From<DateTime<Utc>> for NumericDate {
    fn from(date_time: DateTime<Utc>) -> Self {
        Self::new(date_time.timestamp())
    }
}

impl Display for NumericDate {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.seconds_since_epoch)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use std::time::UNIX_EPOCH;

    use super::NumericDate;

    #[test]
    fn reads_the_seconds_of_a_system_time() {
        assert_eq!(
            NumericDate::from_system_time(UNIX_EPOCH + Duration::from_millis(1_500))
                .expect("the time follows the epoch"),
            NumericDate::new(1)
        );
    }

    #[test]
    fn rejects_a_system_time_before_the_epoch() {
        assert!(
            NumericDate::from_system_time(UNIX_EPOCH - Duration::from_secs(1))
                .expect_err("the time precedes the epoch")
                .to_string()
                .starts_with("the system clock reads a time before the unix epoch: ")
        );
    }

    #[test]
    fn rejects_a_duration_beyond_the_numeric_date_range() {
        assert!(
            NumericDate::from_duration_since_epoch(Duration::from_secs(u64::MAX))
                .expect_err("the duration exceeds the numeric date range")
                .to_string()
                .starts_with("the system clock reads a time beyond the numeric date range: ")
        );
    }

    #[test]
    fn displays_the_seconds_since_the_epoch() {
        assert_eq!(NumericDate::new(1_700_000_000).to_string(), "1700000000");
    }
}
