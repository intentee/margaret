use serde::Deserialize;
use serde::Serialize;

use crate::authorization_grant::AuthorizationGrant;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PendingAuthorization {
    pub grant: AuthorizationGrant,
    pub state: Option<String>,
}
