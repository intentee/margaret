use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::is_expired::IsExpired;

#[derive(Clone, Deserialize, Serialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl IsExpired for AccessTokenClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> bool {
        self.exp < now.timestamp()
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use chrono::Utc;
    use uuid::Uuid;

    use super::AccessTokenClaims;
    use crate::is_expired::IsExpired;

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(secs, 0).expect("a valid timestamp")
    }

    fn claims(exp: i64) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: Uuid::from_u128(1),
            exp,
            iat: 0,
        }
    }

    #[test]
    fn reports_expiry_relative_to_now() {
        assert!(!claims(101).is_expired(at(100)));
        assert!(claims(99).is_expired(at(100)));
        assert!(!claims(100).is_expired(at(100)));
    }
}
