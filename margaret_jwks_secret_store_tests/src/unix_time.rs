use chrono::DateTime;
use chrono::Utc;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn unix_time(seconds: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(seconds, 0).expect("a valid unix timestamp")
}
