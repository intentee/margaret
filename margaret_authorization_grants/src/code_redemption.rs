use uuid::Uuid;

use crate::authorization_grant::AuthorizationGrant;

#[derive(Debug, Eq, PartialEq)]
pub enum CodeRedemption<Decided> {
    AlreadyRedeemed {
        family: Uuid,
        grant: Box<AuthorizationGrant>,
    },
    Redeemed(Decided),
    Unknown,
}
