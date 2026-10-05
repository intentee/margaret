use serde::Deserialize;
use validator::Validate;
use validator::ValidationErrors;

use margaret_oauth_vocabulary::token_type_hint::TokenTypeHint;

#[derive(Debug, Deserialize)]
pub struct TokenSubmission {
    pub client_id: Option<String>,
    pub token: String,
    pub token_type_hint: Option<TokenTypeHint>,
}

impl Validate for TokenSubmission {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}
