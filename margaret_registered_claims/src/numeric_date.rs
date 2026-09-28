use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct NumericDate {
    seconds_since_epoch: i64,
}

impl NumericDate {
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
    use super::NumericDate;

    #[test]
    fn displays_the_seconds_since_the_epoch() {
        assert_eq!(NumericDate::new(1_700_000_000).to_string(), "1700000000");
    }
}
