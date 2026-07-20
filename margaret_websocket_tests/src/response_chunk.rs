use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseChunk {
    pub text: String,
}
