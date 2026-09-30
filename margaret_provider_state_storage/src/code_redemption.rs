use crate::authorization_grant::AuthorizationGrant;

#[derive(Debug, Eq, PartialEq)]
pub enum CodeRedemption {
    Redeemed(Box<AuthorizationGrant>),
    Refused,
    Replayed,
    Unknown,
}
