use chrono::DateTime;
use chrono::Utc;

#[must_use]
pub fn test_instant(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0).expect("the test timestamp is in range")
}
