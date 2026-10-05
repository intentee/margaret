#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshRotation {
    Replayed,
    Revoked,
    Rotated,
}
