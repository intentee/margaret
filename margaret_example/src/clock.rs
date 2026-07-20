use chrono::DateTime;
use chrono::Utc;

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}
