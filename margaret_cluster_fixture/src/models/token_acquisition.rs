use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

use margaret::framework::macros::model;

use crate::models::token_acquisition_outcome::TokenAcquisitionOutcome;

#[model(table = "token_acquisitions")]
pub struct TokenAcquisition {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub attempted_at: DateTime<Utc>,
    #[column]
    pub outcome: TokenAcquisitionOutcome,
}
