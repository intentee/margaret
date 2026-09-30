use crate::pending_authorization::PendingAuthorization;

#[derive(Debug, Eq, PartialEq)]
pub enum DecidedAuthorization {
    Approved(Box<PendingAuthorization>),
    Denied(Box<PendingAuthorization>),
    Unknown,
}
