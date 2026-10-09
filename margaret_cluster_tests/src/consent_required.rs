use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ConsentRequired {
    pub consent: Uuid,
}
