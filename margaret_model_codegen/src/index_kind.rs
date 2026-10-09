#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IndexKind {
    Plain,
    PrimaryKey,
    Unique,
}
