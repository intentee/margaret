#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeadlineWake {
    Cancelled,
    Reached,
}
