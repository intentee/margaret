use serde::Deserialize;

#[derive(Deserialize)]
pub struct StoredMessage {
    pub body: String,
}
