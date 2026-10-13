#[derive(Debug, Eq, PartialEq)]
pub struct UniqueConstraint {
    pub columns: &'static [&'static str],
}
