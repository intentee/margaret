use std::collections::BTreeSet;

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use margaret_oauth_vocabulary::code_challenge::CodeChallenge;
use margaret_oauth_vocabulary::scope::Scope;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AuthorizationGrant {
    pub auth_time: DateTime<Utc>,
    pub client_id: String,
    pub code_challenge: CodeChallenge,
    pub nonce: Option<String>,
    pub redirect_uri: Url,
    pub scopes: BTreeSet<Scope>,
    pub subject: Uuid,
}
