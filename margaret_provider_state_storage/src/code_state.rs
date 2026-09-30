use std::sync::Arc;

use uuid::Uuid;

use crate::authorization_grant::AuthorizationGrant;

#[derive(Clone)]
pub(crate) enum CodeState {
    Issued(Arc<AuthorizationGrant>),
    Redeemed { family: Uuid },
}
