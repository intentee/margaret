use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::forms::consent_choice::ConsentChoice;

#[derive(Deserialize, Validate)]
pub struct ConsentForm {
    pub decision: ConsentChoice,
    pub id: Uuid,
}
