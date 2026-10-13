use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshRotation {
    Rotated,
    Superseded { family: Uuid },
    Unknown,
}
