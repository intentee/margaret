use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct SessionRefreshAnswer {
    pub access_token: String,
}
