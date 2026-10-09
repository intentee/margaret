use chrono::DateTime;
use chrono::Utc;

pub fn joined_at() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 123_456_000).expect("the fixed instant is in range")
}
