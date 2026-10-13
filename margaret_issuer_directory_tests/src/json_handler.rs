use std::sync::Arc;

use serde_json::Value;

use margaret_http::head_handler::HeadHandler;
use margaret_http_tests::static_handler::StaticHandler;

#[must_use]
pub fn json_handler(status: u16, document: &Value) -> Arc<dyn HeadHandler> {
    Arc::new(StaticHandler {
        body: document.to_string().into_bytes(),
        content_type: "application/json",
        status,
    })
}
