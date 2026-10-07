use crate::assertion_refusal::AssertionRefusal;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssertionSpending {
    Refused(AssertionRefusal),
    Spent,
}
