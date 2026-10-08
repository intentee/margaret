use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ConsentRequired {
    pub consent: Uuid,
}
