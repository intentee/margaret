use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct SubjectAnswer {
    pub subject: String,
}
