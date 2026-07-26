use chrono::DateTime;
use chrono::Utc;

pub trait IsExpired {
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool>;
}
