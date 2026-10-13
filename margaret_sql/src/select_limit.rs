#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectLimit {
    Rows(u64),
    Unlimited,
}
