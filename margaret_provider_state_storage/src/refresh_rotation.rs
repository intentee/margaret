use crate::refresh_family::RefreshFamily;

#[derive(Debug, Eq, PartialEq)]
pub enum RefreshRotation {
    ForeignClient,
    Replayed,
    Rotated(RefreshFamily),
    ScopeExceeded,
    Unknown,
}
