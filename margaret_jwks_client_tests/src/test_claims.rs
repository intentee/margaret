use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use margaret_identity_session::is_expired::IsExpired;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct TestClaims {
    pub exp: i64,
    pub sub: String,
}

impl IsExpired for TestClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> bool {
        self.exp < now.timestamp()
    }
}
