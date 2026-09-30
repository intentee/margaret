use serde_json::Value;
use serde_json::json;

pub struct TokenAnswer {
    pub body: Value,
    pub status: u16,
}

impl TokenAnswer {
    #[must_use]
    pub fn issued(id_token: &str) -> Self {
        Self {
            body: json!({
                "access_token": "2YotnFZFEjr1zCsicMWpAA",
                "id_token": id_token,
                "token_type": "Bearer",
            }),
            status: 200,
        }
    }
}
