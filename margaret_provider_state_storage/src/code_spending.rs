#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeSpending {
    Expired,
    Replayed,
    Spent,
}
