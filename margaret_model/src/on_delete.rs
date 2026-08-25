#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnDelete {
    Cascade,
    NoAction,
    Restrict,
    SetDefault,
    SetNull,
}
