#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsentPolicy {
    Implicit,
    Prompted,
}
