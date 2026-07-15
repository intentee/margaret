use serde::Deserialize;

#[derive(Deserialize)]
pub struct JoinTokenOutput {
    pub value: String,
}
