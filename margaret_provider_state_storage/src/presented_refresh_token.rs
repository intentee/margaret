use crate::refresh_family::RefreshFamily;

#[derive(Debug, Eq, PartialEq)]
pub enum PresentedRefreshToken {
    Current(RefreshFamily),
    Replayed,
    Unknown,
}
