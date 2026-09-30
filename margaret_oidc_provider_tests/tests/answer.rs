use reqwest::header::HeaderMap;
use serde_json::Value;

pub struct Answer {
    pub body: Value,
    pub headers: HeaderMap,
    pub status: u16,
}

impl Answer {
    pub async fn of(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let is_json = headers
            .get("content-type")
            .is_some_and(|content_type| content_type == "application/json");
        let bytes = response
            .bytes()
            .await
            .expect("the provider answers with a body");

        Self {
            body: if is_json {
                serde_json::from_slice(&bytes).expect("the provider answers with json")
            } else {
                Value::String(
                    String::from_utf8(bytes.to_vec()).expect("the provider answers with text"),
                )
            },
            headers,
            status,
        }
    }

    pub fn header(&self, name: &str) -> &str {
        self.headers
            .get(name)
            .expect("the answer carries the header")
            .to_str()
            .expect("the header is visible ascii")
    }
}
