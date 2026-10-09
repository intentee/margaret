use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::consent_choice::ConsentChoice;

#[derive(Deserialize, Validate)]
pub struct ConsentSubmission {
    pub decision: ConsentChoice,
    pub id: Uuid,
}
