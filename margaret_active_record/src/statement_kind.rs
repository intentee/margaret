#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatementKind {
    Delete,
    Insert,
    Select,
    Update,
}
