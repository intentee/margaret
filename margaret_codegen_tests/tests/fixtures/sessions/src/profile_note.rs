use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ProfileNote {
    #[validate(length(min = 1))]
    pub text: String,
}
