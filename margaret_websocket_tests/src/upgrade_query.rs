use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpgradeQuery {
    #[validate(length(min = 1))]
    pub label: String,
}
