use chrono::DateTime;
use chrono::Utc;

/// # Panics
///
/// Panics when the clock reads a time beyond the representable range.
#[must_use]
pub fn contract_moment() -> DateTime<Utc> {
    DateTime::from_timestamp(Utc::now().timestamp(), 0)
        .expect("the clock reads a representable time")
}
