#[derive(Debug)]
pub(crate) enum FieldIndex {
    Absent,
    Derived,
    Explicit(String),
}
