use crate::pending_authorization::PendingAuthorization;

#[derive(Debug, Eq, PartialEq)]
pub enum PendingAuthorizationTake {
    Absent,
    Taken(Box<PendingAuthorization>),
}
