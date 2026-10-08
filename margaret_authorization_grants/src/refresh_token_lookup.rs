use uuid::Uuid;

use crate::refresh_family::RefreshFamily;

#[derive(Debug, Eq, PartialEq)]
pub enum RefreshTokenLookup {
    Current { family: Uuid, record: RefreshFamily },
    Superseded { family: Uuid },
    Unknown,
}
