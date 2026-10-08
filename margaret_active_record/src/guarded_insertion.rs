#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardedInsertion {
    Inserted,
    Refused,
}
