use std::time::Duration;

use chrono::Utc;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use margaret_registered_claims::numeric_date::NumericDate;

#[must_use]
pub fn assertion_claims(client_id: &str, audience: &str) -> Value {
    json!({
        "aud": audience,
        "exp": NumericDate::from(Utc::now()).after(Duration::from_mins(1)).seconds_since_epoch(),
        "iss": client_id,
        "jti": Uuid::new_v4().to_string(),
        "sub": client_id,
    })
}
