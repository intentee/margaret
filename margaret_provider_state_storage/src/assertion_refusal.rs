#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssertionRefusal {
    Expired,
    Replayed,
}
