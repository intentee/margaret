use serde::Deserialize;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope_list::ScopeList;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ResourceAccessTokenClaims {
    pub client_id: ClientId,
    pub scope: ScopeList,
    #[serde(rename = "sub")]
    pub subject: String,
}
