use serde_json::json;

use margaret_http_tests::static_handler::StaticHandler;

#[must_use]
pub fn issued_token_answer(id_token: &str) -> StaticHandler {
    StaticHandler {
        body: json!({
            "access_token": "2YotnFZFEjr1zCsicMWpAA",
            "id_token": id_token,
            "token_type": "Bearer",
        })
        .to_string()
        .into_bytes(),
        content_type: "application/json",
        status: 200,
    }
}
