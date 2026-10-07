use std::time::Duration;

use margaret_registered_claims::numeric_date::NumericDate;

pub enum AssertionRetention {
    Expired,
    Retained(Duration),
}

impl AssertionRetention {
    #[must_use]
    pub fn of(expires_at: NumericDate, now: NumericDate) -> Self {
        match u64::try_from(expires_at.seconds_since_epoch() - now.seconds_since_epoch()) {
            Ok(retained_secs @ 1..) => Self::Retained(Duration::from_secs(retained_secs)),
            Ok(0) | Err(_) => Self::Expired,
        }
    }
}
