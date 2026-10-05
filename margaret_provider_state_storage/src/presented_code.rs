use crate::authorization_grant::AuthorizationGrant;

#[derive(Debug, Eq, PartialEq)]
pub enum PresentedCode {
    Issued(Box<AuthorizationGrant>),
    Replayed,
    Unknown,
}
