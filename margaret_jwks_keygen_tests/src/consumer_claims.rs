use serde::Deserialize;
use serde::Serialize;

use crate::consumer_grant::ConsumerGrant;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ConsumerClaims {
    pub aud: Vec<String>,
    pub env: String,
    pub exp: usize,
    pub groups: Option<Vec<String>>,
    pub iat: usize,
    pub idp: String,
    pub is_service_account: Option<bool>,
    pub iss: String,
    pub name: String,
    pub preferred_username: String,
    pub resources: Option<Vec<ConsumerGrant>>,
    pub sub: String,
}
